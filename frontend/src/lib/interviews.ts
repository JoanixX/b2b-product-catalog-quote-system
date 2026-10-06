export type Field = { key: string; label: string; hint?: string; multiline?: boolean };
export const businessFields: Field[] = [
  { key: 'services', label: 'Servicios principales' },
  { key: 'volume', label: 'Pacientes o clientes por día', hint: 'Una aproximación es suficiente.' },
  { key: 'team', label: 'Equipo y funciones' },
  { key: 'tools', label: 'Herramientas actuales', hint: 'Excel, WhatsApp, papel, sistemas…' },
  { key: 'administration', label: 'Procesos administrativos más demandantes', multiline: true },
];
export const workflowFields: Field[] = [
  { key: 'name', label: 'Nombre del proceso', hint: 'Citas, recepción, pagos, exámenes, administración…' },
  { key: 'start', label: '¿Qué lo inicia?' },
  { key: 'people', label: 'Responsables y funciones' },
  { key: 'steps', label: 'Pasos, en orden', hint: 'Quién hace qué → qué ocurre después.', multiline: true },
  { key: 'information', label: 'Información que reciben y dónde la registran' },
  { key: 'tools', label: 'Herramientas utilizadas' },
  { key: 'end', label: '¿Cómo termina? ¿Qué resultado entrega?' },
  { key: 'exceptions', label: '¿Qué ocurre cuando algo falla?', multiline: true },
];
export const problemFields: Field[] = [
  { key: 'name', label: 'Problema concreto' },
  { key: 'example', label: 'Último caso real', hint: 'Qué pasó, quién intervino y cómo lo resolvieron. Sin datos de pacientes.', multiline: true },
  { key: 'cause', label: 'Causa observada o por confirmar' },
  { key: 'frequency', label: 'Frecuencia', hint: 'Por ejemplo: 4 veces al día, o pendiente de medir.' },
  { key: 'impact', label: 'Impacto aproximado', hint: 'Minutos por caso, dinero, errores, reclamos o pacientes perdidos.' },
  { key: 'current', label: 'Cómo lo resuelven hoy y qué han intentado', multiline: true },
  { key: 'expected', label: 'Resultado esperado y cómo medirlo' },
  { key: 'restrictions', label: 'Restricciones y datos necesarios' },
  { key: 'urgency', label: 'Urgencia y disposición a pagar' },
];
export const viabilityFields: Field[] = [
  { key: 'success', label: 'Resultado concreto y criterio de éxito', multiline: true },
  { key: 'payment', label: 'Disposición a pagar implementación y mantenimiento' },
  { key: 'budget', label: 'Presupuesto aproximado', hint: 'Registra “por definir” si todavía no existe.' },
  { key: 'decision', label: 'Quién decide y autoriza el piloto' },
  { key: 'users', label: 'Quiénes usarían la solución diariamente' },
  { key: 'systems', label: 'Sistemas que debe respetar o conectar' },
  { key: 'restrictions', label: 'Restricciones técnicas y de operación', hint: 'Internet, equipos, seguridad, privacidad, capacitación.', multiline: true },
  { key: 'safeTrial', label: '¿Se puede probar con datos ficticios o sin información clínica?' },
  { key: 'expansion', label: '¿Otros negocios tienen problemas similares?' },
];
export const closingFields: Field[] = [
  { key: 'validation', label: 'Resumen validado por el entrevistado', multiline: true },
  { key: 'operator', label: 'Persona que ejecuta el proceso para validarlo' },
  { key: 'pilot', label: 'Disposición y condiciones para probar un piloto' },
  { key: 'next', label: 'Fecha o acuerdo para revisar alternativas y costos' },
];
export const proposalFields: Field[] = [
  { key: 'name', label: 'Nombre de la propuesta' },
  { key: 'solution', label: 'Qué se construiría', multiline: true },
  { key: 'benefit', label: 'Beneficio esperado', hint: 'Tiempo, dinero o errores evitados; indica supuestos.' },
  { key: 'feasibility', label: 'Viabilidad: complejidad, datos y riesgos' },
  { key: 'pilot', label: 'Piloto mínimo y criterio para comprobarlo', multiline: true },
  { key: 'commercial', label: 'Costos de implementación y mantenimiento' },
  { key: 'time', label: 'Tiempo estimado' },
  { key: 'scale', label: 'Posibilidad de adaptarlo a otros clientes' },
];
export type Notes = Record<string, string>;
export type Interview = {
  id: string; createdAt: string; updatedAt: string; client: string; sector: string;
  contact: string; date: string; step: number; completed: boolean;
  business: Notes; workflows: Notes[]; problems: Notes[]; priority: string; priorityReason: string;
  viability: Notes; closing: Notes; proposals: Notes[];
};
export const STORAGE_KEY = 'trazo-interviews-v1';
export const stages = [
  { title: 'Inicio', minutes: 2, description: 'Presenta el objetivo y registra con quién conversas.' },
  { title: 'Negocio', minutes: 4, description: 'Ubica el tamaño, las personas y las herramientas.' },
  { title: 'Procesos', minutes: 8, description: 'Recorre dos procesos de principio a fin.' },
  { title: 'Problemas', minutes: 7, description: 'Documenta tres problemas y elige uno prioritario.' },
  { title: 'Viabilidad', minutes: 6, description: 'Confirma valor, restricciones y quién decide.' },
  { title: 'Cierre', minutes: 3, description: 'Valida lo entendido y acuerda el siguiente paso.' },
  { title: 'Propuestas', minutes: 0, description: 'Después de la reunión: compara hasta tres alternativas.' },
];
export function blankNotes(fields: Field[]): Notes { return Object.fromEntries(fields.map(f => [f.key, ''])); }
export function createInterview(): Interview {
  const now = new Date();
  const date = new Intl.DateTimeFormat('en-CA', { timeZone: 'America/Lima', year: 'numeric', month: '2-digit', day: '2-digit' }).format(now);
  return { id: crypto.randomUUID(), createdAt: now.toISOString(), updatedAt: now.toISOString(), client: '', sector: 'Policlínico', contact: '', date,
    step: 0, completed: false, business: blankNotes(businessFields), workflows: [blankNotes(workflowFields), blankNotes(workflowFields)],
    problems: Array.from({ length: 3 }, () => ({ ...blankNotes(problemFields), workflow: '' })), priority: '', priorityReason: '',
    viability: blankNotes(viabilityFields), closing: blankNotes(closingFields), proposals: [] };
}
function filled(value: string) { return Boolean(value.trim()); }
export function checklist(item: Interview) {
  const priority = item.problems[Number(item.priority)];
  return [
    { label: 'Cliente identificado', done: filled(item.client) },
    { label: 'Dos procesos de principio a fin', done: item.workflows.every(w => ['name', 'start', 'people', 'steps', 'information', 'tools', 'end', 'exceptions'].every(k => filled(w[k]))) },
    { label: 'Tres problemas con ejemplos concretos', done: item.problems.every(p => filled(p.name) && filled(p.example) && filled(p.workflow)) },
    { label: 'Un problema prioritario y el motivo', done: item.priority !== '' && Boolean(priority && filled(priority.name)) && filled(item.priorityReason) },
    { label: 'Frecuencia e impacto del prioritario', done: item.priority !== '' && Boolean(priority && filled(priority.frequency) && filled(priority.impact)) },
    { label: 'Restricciones técnicas identificadas', done: filled(item.viability.restrictions) },
    { label: 'Persona que autoriza el piloto', done: filled(item.viability.decision) },
  ];
}
// Reconstruct only known fields. Imports cannot inject HTML or arbitrary properties.
export function parseInterviews(raw: string): Interview[] {
  const parsed: unknown = JSON.parse(raw);
  const records = Array.isArray(parsed) ? parsed : (parsed as { version?: unknown; interviews?: unknown })?.version === 1 ? (parsed as { interviews: unknown }).interviews : null;
  if (!Array.isArray(records) || records.length > 200) throw new Error('El archivo debe ser un respaldo de entrevistas de Trazo (máximo 200).');
  const string = (v: unknown): string => { if (typeof v !== 'string' || v.length > 10000) throw new Error('Hay un campo inválido o demasiado largo.'); return v; };
  const notes = (v: unknown, fields: Field[]): Notes => {
    if (!v || typeof v !== 'object' || Array.isArray(v)) throw new Error('Faltan notas de una sección.');
    return Object.fromEntries(fields.map(f => [f.key, string((v as Notes)[f.key])]));
  };
  const group = (v: unknown, fields: Field[], min: number, max: number): Notes[] => {
    if (!Array.isArray(v) || v.length < min || v.length > max) throw new Error('La cantidad de procesos, problemas o propuestas es inválida.');
    return v.map(n => notes(n, fields));
  };
  const ids = new Set<string>();
  return records.map(v => {
    if (!v || typeof v !== 'object') throw new Error('Entrevista inválida.');
    const item = v as Interview;
    const id = string(item.id);
    if (!/^[\w-]{1,80}$/.test(id) || ids.has(id)) throw new Error('El respaldo contiene identificadores inválidos o repetidos.');
    ids.add(id);
    const createdAt = string(item.createdAt), updatedAt = string(item.updatedAt), date = string(item.date);
    if (!Number.isFinite(Date.parse(createdAt)) || !Number.isFinite(Date.parse(updatedAt)) || !/^\d{4}-\d{2}-\d{2}$/.test(date)) throw new Error('Fecha inválida.');
    if (!Number.isInteger(item.step) || item.step < 0 || item.step >= stages.length || typeof item.completed !== 'boolean') throw new Error('Estado de entrevista inválido.');
    const priority = string(item.priority);
    if (!['', '0', '1', '2'].includes(priority)) throw new Error('Prioridad inválida.');
    const result: Interview = { id, createdAt, updatedAt, date, client: string(item.client), sector: string(item.sector), contact: string(item.contact),
      step: item.step, completed: item.completed, business: notes(item.business, businessFields), workflows: group(item.workflows, workflowFields, 2, 2),
      problems: group(item.problems, [...problemFields, { key: 'workflow', label: 'Proceso' }], 3, 3), priority, priorityReason: string(item.priorityReason),
      viability: notes(item.viability, viabilityFields), closing: notes(item.closing, closingFields),
      proposals: group(item.proposals, [...proposalFields, { key: 'problem', label: 'Problema' }], 0, 3) };
    if (result.problems.some(p => !['', '0', '1'].includes(p.workflow)) || result.proposals.some(p => !['', '0', '1', '2'].includes(p.problem))) throw new Error('La relación con un proceso o problema es inválida.');
    result.completed = result.completed && checklist(result).every(c => c.done);
    return result;
  });
}
export function readInterviews(): Interview[] {
  try { const raw = localStorage.getItem(STORAGE_KEY); return raw ? parseInterviews(raw) : []; }
  catch { throw new Error('No se pudieron leer las entrevistas. Los datos guardados no se han reemplazado. Descarga el respaldo para recuperarlos.'); }
}
export function saveInterviews(items: Interview[]) {
  try { localStorage.setItem(STORAGE_KEY, JSON.stringify(items)); }
  catch { throw new Error('No se pudo guardar. Mantén esta pestaña abierta y descarga un respaldo de tus notas.'); }
}
export function interviewSummary(item: Interview): string {
  const value = (s: string) => s.trim() || 'Pendiente';
  const section = (name: string, fields: Field[], data: Notes) => `\n## ${name}\n\n${fields.map(f => `**${f.label}:** ${value(data[f.key])}`).join('\n\n')}\n`;
  const priority = item.priority === '' ? undefined : item.problems[Number(item.priority)];
  let text = `# Entrevista comercial — ${value(item.client)}\n\nFecha: ${item.date}\n\nSector: ${value(item.sector)}\n\nEntrevistado y función: ${value(item.contact)}\n\nEstado: ${item.completed ? 'Reunión completada' : 'Borrador'}\n`;
  text += section('Negocio', businessFields, item.business);
  item.workflows.forEach((w, i) => { text += section(`Proceso ${i + 1}`, workflowFields, w); });
  item.problems.forEach((p, i) => {
    text += section(`Problema ${i + 1}`, problemFields, p);
    text += `\n**Proceso asociado:** ${p.workflow === '' ? 'Pendiente' : value(item.workflows[Number(p.workflow)]?.name || '')}\n`;
  });
  text += `\n## Prioridad\n\n${priority ? value(priority.name) : 'Pendiente'}\n\nMotivo: ${value(item.priorityReason)}\n`;
  text += section('Viabilidad comercial y técnica', viabilityFields, item.viability);
  text += section('Cierre y siguientes pasos', closingFields, item.closing);
  item.proposals.forEach((p, i) => {
    text += section(`Propuesta ${i + 1}`, proposalFields, p);
    text += `\n**Problema que resuelve:** ${p.problem === '' ? 'Pendiente' : value(item.problems[Number(p.problem)]?.name || '')}\n`;
  });
  text += `\n## Información mínima\n\n${checklist(item).map(c => `- [${c.done ? 'x' : ' '}] ${c.label}`).join('\n')}\n`;
  return text;
}
