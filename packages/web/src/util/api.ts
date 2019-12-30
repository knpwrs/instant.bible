import { stringify } from 'qs';
import {
  verseKeyToString,
  textToTranslationsObject,
  topTranslation,
} from './proto';
import { instantbible as proto } from '../proto';

const endpoint = process.env.IB_ENDPOINT as string;
const headers = { accept: 'application/protobuf' };

export const search = async (q: string) => {
  try {
    const query = stringify({ q });
    const res = await fetch(`${endpoint}?${query}`, {
      headers,
    });
    const buf = await res.arrayBuffer();
    const decoded = proto.service.Response.decode(new Uint8Array(buf));

    return decoded.results.map(res => ({
      key: verseKeyToString(res.key),
      text: textToTranslationsObject(res.text),
      topTranslation: topTranslation(res.translationScores),
    }));
  } catch (e) {
    console.error(e);
    return null;
  }
};
