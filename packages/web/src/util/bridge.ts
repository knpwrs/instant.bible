import * as idb from './keyval';

type WasmType = typeof import('../wasm');

let wasm: WasmType | null = null;

export const getWasm = () => wasm;

export const setWasm = (w: WasmType) => {
  wasm = w;
};

const localBytesKey = 'indexBytes';

export const getLocalBytes = () => idb.getLarge(localBytesKey);

export const setLocalBytes = (bytes: Uint8Array) =>
  idb.setLarge(localBytesKey, bytes);

export const delLocalBytes = () => idb.delLarge(localBytesKey);
