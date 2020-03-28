import * as idb from 'idb-keyval';

type WasmType = typeof import('../wasm');

let wasm: WasmType | null = null;

export const getWasm = () => wasm;

export const setWasm = (w: WasmType) => {
  wasm = w;
};

const localBytesKey = 'indexBytes';

export const getLocalBytes = () =>
  idb.get<Uint8Array | undefined>(localBytesKey);

export const setLocalBytes = (bytes: Uint8Array) =>
  idb.set(localBytesKey, bytes);

export const delLocalBytes = () => idb.del(localBytesKey);
