import { isNumber, max } from 'lodash';
import { instantbible as proto } from '../../proto';
import en from './en';

export const bookToString = (book?: proto.data.Book | null) => {
  if (!isNumber(book)) throw new Error('Invalid book!');

  return en[book];
};

export const verseKeyToString = (key?: proto.data.IVerseKey | null) => {
  if (!key) throw new Error('Invalid key!');

  return `${bookToString(key.book)} ${key.chapter}:${key.verse}`;
};

const translationToString = (i: number) => {
  const t = proto.data.Translation[i];

  return t;
};

export const textToTranslationsObject = (text?: string[] | null) => {
  if (!text) throw new Error('Invalid text!');

  return text.reduce(
    (acc, txt, i) => ({
      ...acc,
      [translationToString(i)]: txt,
    }),
    {},
  );
};

export const topTranslation = (translationScores?: number[] | null) => {
  if (!translationScores) throw new Error('Invalid translation scores!');

  const m = max(translationScores) || -1;
  const i = translationScores.indexOf(m) || 0;
  return translationToString(i);
};