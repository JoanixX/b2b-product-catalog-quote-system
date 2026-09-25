import {test,beforeEach} from 'node:test';
import assert from 'node:assert/strict';
import {readQuotes,resetQuotes,saveQuotes,sampleQuote} from '../src/lib/demo.ts';
let raw;
beforeEach(()=>{raw=null;globalThis.localStorage={getItem:()=>raw,setItem:(_,value)=>{raw=value;}}});
test('empty session, corrupted data and reset recovery',()=>{
  assert.deepEqual(readQuotes(),[]);
  raw='{broken';assert.throws(readQuotes,/Restablécela/);
  resetQuotes();assert.equal(readQuotes()[0].stage,'recibida');assert.equal(readQuotes().length,1);
});
test('untrusted stored stages and dates cannot reach rendering',()=>{
  for(const change of [{stage:'constructor'},{createdAt:'invalid'},{quantity:10001},{unitPrice:Infinity}]){
    raw=JSON.stringify([{...sampleQuote(),...change}]);assert.throws(readQuotes,/Restablécela/);
  }
});
test('blocked storage never reports save success',()=>{
  globalThis.localStorage.setItem=()=>{throw Error('QuotaExceededError')};
  assert.throws(()=>saveQuotes([sampleQuote()]),/No se pudieron guardar/);
  assert.throws(resetQuotes,/No se pudieron guardar/);
});
