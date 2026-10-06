import { test, beforeEach } from 'node:test';
import assert from 'node:assert/strict';
import { createInterview, checklist, parseInterviews, readInterviews, saveInterviews, interviewSummary, STORAGE_KEY } from '../src/lib/interviews.ts';
let storage;
beforeEach(() => {
  storage = new Map();
  globalThis.localStorage = { getItem: key => storage.get(key) ?? null, setItem: (key, value) => storage.set(key, value) };
});
test('independent clients and complete backup round trip preserve notes and proposals', () => {
  const a = createInterview(), b = createInterview(); a.client = 'Policlínico Uno'; b.client = 'Cliente Dos';
  a.workflows[0].steps = 'Solicitud → disponibilidad → confirmación';
  a.problems[0].frequency = '4 veces al día';
  saveInterviews([a, b]);
  const loaded = readInterviews();
  assert.equal(loaded[0].workflows[0].steps, a.workflows[0].steps);
  assert.equal(loaded[1].workflows[0].steps, '');
  assert.notEqual(a.id, b.id);
  assert.deepEqual(parseInterviews(JSON.stringify({ version: 1, interviews: loaded })), loaded);
  assert.match(interviewSummary(a), /Solicitud → disponibilidad → confirmación/);
});
test('completion requires actual processes, examples, priority evidence, constraints and decision maker', () => {
  const item = createInterview();
  assert.equal(checklist(item).some(c => c.done), false);
  item.client = 'Cliente';
  for (const w of item.workflows) for (const key of Object.keys(w)) w[key] = 'Registrado';
  for (const p of item.problems) { p.name = 'Problema'; p.example = 'Caso real'; p.workflow = '0'; }
  item.priority = '0'; item.priorityReason = 'Mayor impacto'; item.problems[0].frequency = '2 por día';
  item.problems[0].impact = '20 minutos'; item.viability.restrictions = 'Internet inestable'; item.viability.decision = 'Administrador';
  assert.equal(checklist(item).every(c => c.done), true);
  item.completed = true;
  item.problems[0].impact = '';
  assert.equal(parseInterviews(JSON.stringify([item]))[0].completed, false);
});
test('malformed backups, duplicate IDs and invalid relationships cannot overwrite saved data', () => {
  const item = createInterview(); saveInterviews([item]); const original = storage.get(STORAGE_KEY);
  for (const invalid of ['{broken', JSON.stringify([item, item]), JSON.stringify([{ ...item, step: 7 }]), JSON.stringify([{ ...item, proposals: [{ name: 'x' }] }]), JSON.stringify([{ ...item, priority: '__proto__' }])]) assert.throws(() => parseInterviews(invalid));
  const altered = structuredClone(item); altered.problems[0].workflow = '99';
  assert.throws(() => parseInterviews(JSON.stringify([altered])));
  assert.equal(storage.get(STORAGE_KEY), original);
});
test('blocked or corrupted storage reports failure and preserves original contents', () => {
  storage.set(STORAGE_KEY, '{broken');
  assert.throws(readInterviews, /no se han reemplazado/);
  assert.equal(storage.get(STORAGE_KEY), '{broken');
  globalThis.localStorage.setItem = () => { throw new Error('QuotaExceededError'); };
  assert.throws(() => saveInterviews([createInterview()]), /No se pudo guardar/);
});
