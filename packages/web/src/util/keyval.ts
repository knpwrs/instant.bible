import * as idb from 'idb-keyval';

// This module is a wrapper around idb-keyval to allow storing large buffers in
// IndexedDB. This is accomplished by splitting the buffers into 100MB chunks
// which are stored under separate keys.

const ONE_HUNDRED_MB = 100 * 1000 * 1000;

const getChunkCount = (bytes: number) => Math.ceil(bytes / ONE_HUNDRED_MB);
const getChunkKey = (key: string, i: number) => `${key}--chunk-${i}`;
const getBytesKey = (key: string) => `${key}--bytes`;

export const setLarge = async (key: string, bytes: Uint8Array) => {
  const chunkCount = getChunkCount(bytes.length);

  await idb.set(getBytesKey(key), bytes.length);

  for (let i = 0; i < chunkCount; i += 1) {
    await idb.set(
      getChunkKey(key, i),
      bytes.slice(
        i * ONE_HUNDRED_MB,
        Math.min(i * ONE_HUNDRED_MB + ONE_HUNDRED_MB, bytes.length),
      ),
    );
  }
};

export const getLarge = async (key: string) => {
  const byteLength: number | undefined = await idb.get(getBytesKey(key));

  if (!byteLength) {
    return;
  }

  const bytes = new Uint8Array(byteLength);
  const chunkCount = Math.ceil(byteLength / ONE_HUNDRED_MB);

  for (let i = 0; i < chunkCount; i += 1) {
    bytes.set(await idb.get(getChunkKey(key, i)), i * ONE_HUNDRED_MB);
  }

  return bytes;
};

export const delLarge = async (key: string) => {
  const bytesKey = getBytesKey(key);
  const byteLength: number | undefined = await idb.get(bytesKey);

  if (!byteLength) {
    return;
  }

  await idb.del(bytesKey);

  const chunkCount = Math.ceil(byteLength / ONE_HUNDRED_MB);

  for (let i = 0; i < chunkCount; i += 1) {
    await idb.del(getChunkKey(key, i));
  }
};
