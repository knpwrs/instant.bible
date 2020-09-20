import * as idb from './keyval';

const localBytesKey = 'indexBytes';

export const getLocalBytes = () => idb.getLarge(localBytesKey);

export const setLocalBytes = (bytes: Uint8Array) =>
  idb.setLarge(localBytesKey, bytes);

export const delLocalBytes = () => idb.delLarge(localBytesKey);
