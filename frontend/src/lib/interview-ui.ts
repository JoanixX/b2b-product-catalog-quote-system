import { escapeHtml as esc } from './demo';
import { STORAGE_KEY, stages, businessFields, workflowFields, problemFields, viabilityFields, closingFields, proposalFields, blankNotes, createInterview, checklist, parseInterviews, readInterviews, saveInterviews, interviewSummary, type Interview, type Notes, type Field } from './interviews';

const el = <T extends HTMLElement = HTMLElement>(id: string) => document.getElementById(id) as T;
let interviews: Interview[] = [];
let active: Interview | undefined;
let record = 0;
let storageHealthy = true;
let loadFailed = false;
let clockSeconds = 0;
let clockStarted = 0;
let clockRunning = false;
const content = el('step-content');
const dialog = el<HTMLDialogElement>('new-dialog');

function announce(message: string) { el('announcement').textContent = message; }
function showError(message: string) { el('interview-error').textContent = message; el('interview-error').hidden = false; }
function persist() {
  if (!active || loadFailed) return;
  active.updatedAt = new Date().toISOString();
  if (active.completed && !checklist(active).every(c => c.done)) active.completed = false;
  try {
    saveInterviews(interviews); storageHealthy = true; el('interview-error').hidden = true;
    el('save-state').textContent = active.completed ? 'Reunión completada · guardada en este navegador' : 'Guardado en este navegador';
  } catch (error) { storageHealthy = false; el('save-state').textContent = 'Sin guardar · descarga un respaldo'; showError((error as Error).message); }
  el('client-title').textContent = active.client || 'Cliente por identificar';
  renderLibrary();
}
function fieldMarkup(fields: Field[], data: Notes, prefix: string): string {
  return fields.map(f => `<label class="field ${f.multiline ? 'wide' : ''}">${esc(f.label)}${f.multiline
    ? `<textarea aria-label="${esc(f.label)}" ${f.hint ? `aria-describedby="hint-${prefix}-${f.key}"` : ''} data-path="${prefix}.${f.key}" maxlength="10000">${esc(data[f.key])}</textarea>`
    : `<input aria-label="${esc(f.label)}" ${f.hint ? `aria-describedby="hint-${prefix}-${f.key}"` : ''} data-path="${prefix}.${f.key}" value="${esc(data[f.key])}" maxlength="10000" />`}${f.hint ? `<small id="hint-${prefix}-${f.key}">${esc(f.hint)}</small>` : ''}</label>`).join('');
}
function selectMarkup(label: string, path: string, value: string, options: { value: string; label: string }[]) {
  return `<label class="field">${esc(label)}<select aria-label="${esc(label)}" data-path="${path}"><option value="">Por definir</option>${options.map(o => `<option value="${esc(o.value)}" ${value === o.value ? 'selected' : ''}>${esc(o.label)}</option>`).join('')}</select></label>`;
}
function help(questions: string[]) { return `<details class="question-help"><summary>Preguntas de apoyo</summary><ul>${questions.map(q => `<li>${esc(q)}</li>`).join('')}</ul></details>`; }
function prompt(text: string) { return `<blockquote class="prompt"><p>“${esc(text)}”</p></blockquote>`; }
function tabs(type: string, count: number, label: string) {
  return `<div class="record-tabs" role="group" aria-label="${label}">${Array.from({ length: count }, (_, i) => `<button type="button" data-record="${i}" data-kind="${type}" aria-pressed="${record === i}">${label} ${i + 1}</button>`).join('')}</div>`;
}
function checklistMarkup(item: Interview) {
  return `<ul class="checklist">${checklist(item).map(c => `<li class="${c.done ? 'done' : ''}"><span class="check-icon" aria-hidden="true">${c.done ? '✓' : '○'}</span><span>${esc(c.label)}${c.done ? ' · registrado' : ' · pendiente'}</span></li>`).join('')}</ul>`;
}
function renderLibrary() {
  el('interview-count').textContent = String(interviews.length);
  el('interview-list').innerHTML = interviews.length ? interviews.map(i => `<div class="interview-row"><button type="button" data-open="${i.id}" ${i.id === active?.id ? 'aria-current="true"' : ''}><strong>${esc(i.client || 'Cliente por identificar')}</strong><small>${esc(i.sector)} · ${esc(i.date)} · ${i.completed ? 'Reunión completada' : 'Borrador'}${i.id === active?.id ? ' · abierta' : ''}</small></button><button type="button" class="text-button" data-delete="${i.id}" aria-label="Eliminar entrevista de ${esc(i.client)}">Eliminar</button></div>`).join('') : '<p class="muted">Todavía no hay entrevistas. Crea una para empezar.</p>';
}
function renderStep(focus = false) {
  if (!active) return;
  const s = active.step;
  el('interview-editor').hidden = false; el('empty-interviews').hidden = true; el('summary-panel').hidden = true;
  el('client-title').textContent = active.client || 'Cliente por identificar';
  el('save-state').textContent = storageHealthy ? active.completed ? 'Reunión completada · guardada en este navegador' : 'Guardado en este navegador' : 'Sin guardar · descarga un respaldo';
  el('step-counter').textContent = s === 6 ? 'Después de la reunión' : `Etapa ${s + 1} de 6 · ${stages[s].minutes} min sugeridos`;
  el('step-title').textContent = stages[s].title;
  el('step-description').textContent = stages[s].description;
  el('stage-progress').style.width = `${Math.min(s + 1, 6) / 6 * 100}%`;
  document.querySelectorAll<HTMLButtonElement>('[data-step]').forEach(b => { if (Number(b.dataset.step) === s) b.setAttribute('aria-current', 'step'); else b.removeAttribute('aria-current'); });
  el<HTMLButtonElement>('previous-step').disabled = s === 0;
  el('next-step').textContent = s === 6 ? 'Ver resumen →' : s === 5 ? 'Preparar propuestas →' : 'Siguiente →';
  el('step-note').textContent = s === 5 ? 'Las propuestas se preparan después.' : 'Puedes dejar pendientes y volver después.';
  if (s === 0) content.innerHTML = prompt('Quiero entender cómo trabajan actualmente, qué problemas tienen y qué procesos podrían mejorarse. Primero veremos cómo funciona el negocio y después qué necesidades valdría la pena resolver. Con eso prepararé algunas propuestas concretas.') + `<div class="fields"><label class="field">Cliente o negocio<input data-path="client" maxlength="200" value="${esc(active.client)}" /></label><label class="field">Sector<input data-path="sector" maxlength="200" value="${esc(active.sector)}" /></label><label class="field">Entrevistado y función<input data-path="contact" maxlength="200" value="${esc(active.contact)}" /></label><label class="field">Fecha<input type="date" data-path="date" value="${esc(active.date)}" /></label></div>` + help(['Esta guía la completa el entrevistador, durante una conversación.', 'El reloj es opcional: puedes pausarlo para hacer una pausa.', 'No prometas una solución ni sugieras tecnología antes de entender el problema.']);
  if (s === 1) content.innerHTML = `<div class="fields">${fieldMarkup(businessFields, active.business, 'business')}</div>`;
  if (s === 2) content.innerHTML = prompt(active.sector.toLowerCase().includes('policl') ? 'Explícame paso a paso qué ocurre desde que un paciente se comunica con el policlínico hasta que termina su atención.' : 'Explícame paso a paso qué ocurre desde que un cliente se comunica con el negocio hasta que termina su atención.') + tabs('workflows', 2, 'Proceso') + `<div class="fields">${fieldMarkup(workflowFields, active.workflows[record], `workflows.${record}`)}</div>` + help(['¿Quién inicia el proceso y quiénes intervienen?', '¿Qué hace cada persona y qué información recibe?', '¿Qué sucede después y cómo termina?', '¿Qué pasa cuando hay un error, una demora o una excepción?', 'En un policlínico: citas, recepción, pagos, exámenes y administración. Profundiza solo en los dos procesos más problemáticos.']);
  if (s === 3) {
    const p = active.problems[record];
    content.innerHTML = prompt('¿Cuáles son los tres problemas principales actualmente? Cuéntame la última vez que ocurrió uno y cómo lo resolvieron.') + tabs('problems', 3, 'Problema') + `<div class="fields">${selectMarkup('Proceso relacionado', `problems.${record}.workflow`, p.workflow, active.workflows.map((w, i) => ({ value: String(i), label: w.name || `Proceso ${i + 1}` })))}${fieldMarkup(problemFields, p, `problems.${record}`)}</div>` + help(['¿Qué tareas consumen demasiado tiempo?', '¿Dónde ocurren errores, retrasos, pérdidas de información o reclamos?', '¿Qué actividades son manuales o repetitivas?', '¿Qué quieren mejorar que las herramientas actuales no permiten?']) + `<div class="priority-block"><h3>Si solo pudieras resolver uno este mes…</h3>${selectMarkup('Problema prioritario', 'priority', active.priority, active.problems.map((problem, i) => ({ value: String(i), label: problem.name || `Problema ${i + 1}` })))}<label class="field">¿Por qué este problema?<textarea data-path="priorityReason" maxlength="10000">${esc(active.priorityReason)}</textarea></label></div>`;
  }
  if (s === 4) content.innerHTML = `<div class="fields">${fieldMarkup(viabilityFields, active.viability, 'viability')}</div>` + help(['Si la solución demostrara ese resultado, ¿considerarías pagar por implementarla y mantenerla?', '¿Existe presupuesto o alguien más debe autorizarlo?', '¿Cómo sabrías que funciona correctamente?', '¿Podría probarse con datos ficticios o sin información clínica sensible?']);
  if (s === 5) {
    const p = active.priority === '' ? undefined : active.problems[Number(active.priority)];
    content.innerHTML = prompt(`Los problemas identificados son ${active.problems.map(p => p.name || '[por completar]').join(', ')}. El más importante parece ser ${p?.name || '[por priorizar]'}, porque ${active.priorityReason || '[motivo por confirmar]'}. Su impacto es ${p?.impact || '[por estimar]'} y ocurre ${p?.frequency || '[por medir]'}. ¿Lo he entendido correctamente?`) + `<div class="fields">${fieldMarkup(closingFields, active.closing, 'closing')}</div>` + `<details class="question-help"><summary>Compromiso al cerrar</summary><p>“Con esta información prepararé entre una y tres propuestas, considerando beneficio, dificultad, costo y tiempo. Luego elegiríamos una para evaluar un piloto.”</p></details><div class="completion"><h3>¿Tenemos la información mínima?</h3>${checklistMarkup(active)}<button class="button button-primary" type="button" id="complete-interview">${active.completed ? 'Revisar reunión completada' : 'Marcar reunión completada'}</button><p>Los pendientes no impiden guardar ni continuar. Completa esta lista para marcar la reunión como terminada.</p></div>`;
  }
  if (s === 6) {
    record = Math.min(record, Math.max(active.proposals.length - 1, 0));
    content.innerHTML = `<p class="proposal-empty">Prepara estas alternativas después de escuchar al cliente. Vincula cada una con un problema registrado; no inventes necesidades.</p><div class="proposal-actions"><button type="button" class="button button-secondary" id="add-proposal" ${active.proposals.length === 3 ? 'disabled' : ''}>Añadir propuesta ＋</button><span class="muted">${active.proposals.length} de 3</span></div>`;
    if (active.proposals.length) {
      const p = active.proposals[record];
      content.innerHTML += tabs('proposals', active.proposals.length, 'Propuesta') + `<div class="fields">${selectMarkup('Problema que resuelve', `proposals.${record}.problem`, p.problem, active.problems.map((problem, i) => ({ value: String(i), label: problem.name || `Problema ${i + 1}` })))}${fieldMarkup(proposalFields, p, `proposals.${record}`)}</div><p><button type="button" class="text-button" id="remove-proposal">Eliminar esta propuesta</button></p><div id="proposal-comparison">${comparison(active)}</div>`;
    }
  }
  if (focus) el('step-title').focus();
}
function comparison(item: Interview) {
  if (!item.proposals.length) return '';
  const rows = [{ key: 'problem', label: 'Problema' }, ...proposalFields.filter(f => f.key !== 'name')];
  return `<div class="comparison"><table><caption class="sr-only">Comparación de propuestas</caption><thead><tr><th scope="col">Criterio</th>${item.proposals.map((p, i) => `<th scope="col">${esc(p.name || `Propuesta ${i + 1}`)}</th>`).join('')}</tr></thead><tbody>${rows.map(f => `<tr><th scope="row">${esc(f.label)}</th>${item.proposals.map(p => `<td>${esc(f.key === 'problem' ? p.problem === '' ? 'Pendiente' : item.problems[Number(p.problem)]?.name || 'Pendiente' : p[f.key] || 'Pendiente')}</td>`).join('')}</tr>`).join('')}</tbody></table></div>`;
}
function summarySection(title: string, fields: Field[], notes: Notes) {
  return `<section class="summary-section"><h3>${esc(title)}</h3><dl>${fields.map(f => `<div><dt>${esc(f.label)}</dt><dd>${esc(notes[f.key] || 'Pendiente')}</dd></div>`).join('')}</dl></section>`;
}
function renderSummary() {
  if (!active) return;
  el('interview-editor').hidden = true; el('summary-panel').hidden = false;
  const priority = active.priority === '' ? undefined : active.problems[Number(active.priority)];
  let html = summarySection(active.client || 'Cliente por identificar', [{ key: 'sector', label: 'Sector' }, { key: 'contact', label: 'Entrevistado y función' }, { key: 'date', label: 'Fecha' }, { key: 'status', label: 'Estado' }], { sector: active.sector, contact: active.contact, date: active.date, status: active.completed ? 'Reunión completada' : 'Borrador' });
  html += `<section class="summary-section"><h3>Información mínima</h3>${checklistMarkup(active)}</section>`;
  html += summarySection('Negocio', businessFields, active.business);
  active.workflows.forEach((w, i) => { html += summarySection(`Proceso ${i + 1}`, workflowFields, w); });
  active.problems.forEach((p, i) => { html += summarySection(`Problema ${i + 1}`, [{ key: 'processName', label: 'Proceso relacionado' }, ...problemFields], { ...p, processName: p.workflow === '' ? '' : active!.workflows[Number(p.workflow)]?.name || '' }); });
  html += summarySection('Prioridad', [{ key: 'name', label: 'Problema prioritario' }, { key: 'reason', label: 'Motivo' }], { name: priority?.name || '', reason: active.priorityReason });
  html += summarySection('Viabilidad comercial y técnica', viabilityFields, active.viability);
  html += summarySection('Cierre y siguientes pasos', closingFields, active.closing);
  if (active.proposals.length) html += `<section class="summary-section"><h3>Alternativas de solución</h3>${comparison(active)}</section>`;
  el('summary-content').innerHTML = html; el('summary-title').focus();
}
function stopClock() {
  if (clockRunning) clockSeconds += Math.floor((Date.now() - clockStarted) / 1000);
  clockRunning = false; el('toggle-timer').textContent = 'Continuar reloj'; updateClock();
}
function updateClock() {
  const total = clockSeconds + (clockRunning ? Math.floor((Date.now() - clockStarted) / 1000) : 0);
  el('timer').textContent = `${String(Math.floor(total / 60)).padStart(2, '0')}:${String(total % 60).padStart(2, '0')} / 30:00`;
}
function openInterview(id: string) {
  if (active && !storageHealthy && !confirm('Hay notas sin guardar. Descarga un respaldo antes de cambiar. ¿Cambiar de entrevista igualmente?')) return;
  stopClock(); clockSeconds = 0; updateClock(); el('toggle-timer').textContent = 'Iniciar reloj';
  active = interviews.find(i => i.id === id); record = 0;
  el<HTMLDetailsElement>('library').open = false; renderLibrary(); renderStep(true);
}
function download(data: string, name: string, type: string) {
  const url = URL.createObjectURL(new Blob([data], { type })); const link = document.createElement('a');
  link.href = url; link.download = name; link.click(); setTimeout(() => URL.revokeObjectURL(url), 1000);
}
function filename() { return (active?.client || 'entrevista').normalize('NFKD').replace(/[^a-zA-Z0-9_-]/g, '-').slice(0, 70); }
function backup(items: Interview[]) { return JSON.stringify({ version: 1, interviews: items }, null, 2); }
function showNew() { if (loadFailed) { showError('Descarga el respaldo original antes de recuperar las entrevistas; los datos actuales no se reemplazarán.'); return; } dialog.showModal(); el<HTMLInputElement>('new-client').focus(); }

el('new-interview').addEventListener('click', showNew);
el('start-interview').addEventListener('click', showNew);
el('cancel-new').addEventListener('click', () => dialog.close());
el('new-form').addEventListener('submit', e => {
  e.preventDefault();
  const client = el<HTMLInputElement>('new-client').value.trim(); if (!client) { el<HTMLInputElement>('new-client').setCustomValidity('Escribe el nombre del negocio.'); el<HTMLInputElement>('new-client').reportValidity(); return; }
  if (interviews.length >= 200) { showError('Has alcanzado 200 entrevistas. Respalda y elimina las que ya no necesites.'); dialog.close(); return; }
  if (!storageHealthy && !confirm('Hay cambios sin guardar. ¿Crear otra entrevista conservándolos en esta pestaña?')) return;
  const item = createInterview(); item.client = client; item.sector = el<HTMLInputElement>('new-sector').value.trim();
  interviews.unshift(item); active = item; persist(); dialog.close(); el<HTMLFormElement>('new-form').reset(); openInterview(item.id);
});
el('new-client').addEventListener('input', () => el<HTMLInputElement>('new-client').setCustomValidity(''));
document.querySelectorAll<HTMLButtonElement>('[data-step]').forEach(b => b.addEventListener('click', () => { if (active) { active.step = Number(b.dataset.step); record = 0; persist(); renderStep(true); } }));
el('previous-step').addEventListener('click', () => { if (active && active.step > 0) { active.step--; record = 0; persist(); renderStep(true); } });
el('next-step').addEventListener('click', () => { if (!active) return; if (active.step === 6) { renderSummary(); return; } active.step++; record = 0; persist(); renderStep(true); });

content.addEventListener('input', e => {
  if (!active) return;
  const target = e.target as HTMLInputElement | HTMLTextAreaElement | HTMLSelectElement;
  const path = target.dataset.path; if (!path) return;
  // Paths are authored in this module, never taken from an import.
  const parts = path.split('.'); let object: Record<string, unknown> = active as unknown as Record<string, unknown>;
  for (const part of parts.slice(0, -1)) object = object[part] as Record<string, unknown>;
  if (path === 'date' && !/^\d{4}-\d{2}-\d{2}$/.test(target.value)) return;
  object[parts[parts.length - 1]] = target.value; persist();
  if (/^problems\.\d\.name$/.test(path)) {
    const option = content.querySelector<HTMLOptionElement>(`select[data-path="priority"] option[value="${parts[1]}"]`);
    if (option) option.textContent = target.value || `Problema ${Number(parts[1]) + 1}`;
  }
  if (active.step === 6 && el('proposal-comparison')) el('proposal-comparison').innerHTML = comparison(active);
});
content.addEventListener('click', e => {
  const button = (e.target as HTMLElement).closest<HTMLButtonElement>('button'); if (!button || !active) return;
  if (button.dataset.record !== undefined) { record = Number(button.dataset.record); renderStep(); content.querySelector<HTMLInputElement>('input,select')?.focus(); }
  if (button.id === 'complete-interview') {
    if (!checklist(active).every(c => c.done)) { renderStep(); showError('Falta información mínima. Revisa los pendientes de la lista; tus notas siguen guardadas.'); return; }
    active.completed = true; stopClock(); persist(); renderStep(); announce('Reunión marcada como completada. Puedes preparar las propuestas después.');
  }
  if (button.id === 'add-proposal' && active.proposals.length < 3) { active.proposals.push({ ...blankNotes(proposalFields), problem: active.priority }); record = active.proposals.length - 1; persist(); renderStep(); }
  if (button.id === 'remove-proposal' && confirm('¿Eliminar esta propuesta y sus notas?')) { active.proposals.splice(record, 1); record = 0; persist(); renderStep(); }
});
el('interview-list').addEventListener('click', e => {
  const button = (e.target as HTMLElement).closest<HTMLButtonElement>('button'); if (!button) return;
  if (button.dataset.open) openInterview(button.dataset.open);
  if (button.dataset.delete && confirm('¿Eliminar esta entrevista de este navegador? Descarga un respaldo si quieres conservarla.')) {
    const next = interviews.filter(i => i.id !== button.dataset.delete);
    try { saveInterviews(next); interviews = next; el('interview-error').hidden = true; }
    catch (error) { showError((error as Error).message); return; }
    if (active?.id === button.dataset.delete) { active = undefined; stopClock(); if (interviews.length) openInterview(interviews[0].id); else { el('interview-editor').hidden = true; el('summary-panel').hidden = true; el('empty-interviews').hidden = false; } }
    renderLibrary();
  }
});
el('backup-all').addEventListener('click', () => {
  if (loadFailed) { try { download(localStorage.getItem(STORAGE_KEY) || '[]', 'trazo-respaldo-por-recuperar.json', 'application/json'); } catch { showError('El navegador no permite acceder al almacenamiento.'); } return; }
  download(backup(interviews), 'trazo-entrevistas.json', 'application/json');
});
el('import-backup').addEventListener('change', async () => {
  const input = el<HTMLInputElement>('import-backup'); const file = input.files?.[0]; if (!file) return;
  try {
    if (loadFailed) throw new Error('El almacenamiento actual no se puede leer. Conserva el respaldo original y recupéralo en otro navegador antes de importar aquí.');
    if (!storageHealthy) throw new Error('Hay notas sin guardar. Descarga un respaldo antes de importar.');
    if (file.size > 8_000_000) throw new Error('El archivo supera 8 MB. Importa un respaldo más pequeño.');
    const imported = parseInterviews(await file.text());
    // New identifiers preserve both copies, including backups from this browser.
    const next = [...imported.map(i => ({ ...i, id: crypto.randomUUID() })), ...interviews];
    if (next.length > 200) throw new Error('La importación superaría el máximo de 200 entrevistas.');
    saveInterviews(next); interviews = next; renderLibrary();
    if (imported.length) openInterview(next[0].id);
    announce(`${imported.length} entrevistas importadas. Las anteriores se conservaron.`);
  } catch (error) { showError((error as Error).message); } finally { input.value = ''; }
});
el('view-summary').addEventListener('click', renderSummary);
el('close-summary').addEventListener('click', () => renderStep(true));
el('download-notes').addEventListener('click', () => { if (active) download(interviewSummary(active), `${filename()}-notas.md`, 'text/markdown;charset=utf-8'); });
el('download-interview').addEventListener('click', () => { if (active) download(backup([active]), `${filename()}-respaldo.json`, 'application/json'); });
el('print-summary').addEventListener('click', () => { renderSummary(); window.print(); });
el('toggle-timer').addEventListener('click', () => { if (clockRunning) stopClock(); else { clockStarted = Date.now(); clockRunning = true; el('toggle-timer').textContent = 'Pausar reloj'; } });
setInterval(updateClock, 1000);
window.addEventListener('beforeunload', e => { if (!storageHealthy) { e.preventDefault(); e.returnValue = ''; } });
try {
  interviews = readInterviews(); renderLibrary();
  if (interviews.length) openInterview(interviews[0].id);
} catch (error) { loadFailed = true; storageHealthy = false; showError((error as Error).message); el<HTMLDetailsElement>('library').open = true; }
