import { stringify } from 'qs';
import * as PProgress from 'p-progress';
import {
  verseKeyToString,
  verseKeyToObject,
  textToTranslationsObject,
  topTranslation,
} from './proto';
import { getLocalBytes, setLocalBytes } from './index-storeage';
import { instantbible as proto } from '../proto';

const apiServer = process.env.IB_API as string;
const indexUrl = process.env.IB_INDEX_URL as string;
const headers = { accept: 'application/protobuf' };

export const decodeApiResponse = (buf: Uint8Array | ArrayBuffer) => {
  const decoded = proto.service.Response.decode(
    buf instanceof Uint8Array ? buf : new Uint8Array(buf),
  );

  return decoded.results.map((res) => ({
    id: verseKeyToString(res.key),
    key: verseKeyToObject(res.key),
    text: textToTranslationsObject(res.text),
    topTranslation: topTranslation(res.topTranslation),
    highlights: res.highlights || [],
  }));
};

export const search = async (q: string) => {
  try {
    const query = stringify({ q });

    const res = await fetch(`${apiServer}?${query}`, {
      headers,
    });

    const buf = await res.arrayBuffer();

    return decodeApiResponse(buf);
  } catch (e) {
    console.error(e);
    return null;
  }
};

export const getIndexSize = async () => {
  try {
    const res = await fetch(indexUrl, { method: 'HEAD' });

    return parseInt(res.headers.get('Content-Length') || '0', 10);
  } catch (e) {
    debugger;
    return null;
  }
};

export const getIndexBytes = PProgress.fn(async (progress) => {
  const localBytes = await getLocalBytes();

  if (localBytes) {
    return localBytes;
  }

  const res = await fetch(indexUrl);
  const length = parseInt(res.headers.get('Content-Length') || '0', 10);

  if (!length || !res.body) {
    throw new Error('Invalid response');
  }

  const array = new Uint8Array(length);
  let i = 0;

  const reader = res.body.getReader();

  while (true) {
    const { value, done } = await reader.read();
    if (done || !value) {
      break;
    }
    array.set(value, i);
    i += value.length;
    progress(i / length);
  }

  await setLocalBytes(array);

  return array;
});
