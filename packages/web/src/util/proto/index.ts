import { isNumber } from 'lodash';
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

export const verseKeyToObject = (key?: proto.data.IVerseKey | null) => {
  if (!key || !key.book || !key.chapter || !key.verse) {
    throw new Error('Invalid key!');
  }

  return {
    book: key.book,
    chapter: key.chapter,
    verse: key.verse,
  };
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

export const topTranslation = (i?: number | null) => {
  if (!isNumber(i)) throw new Error('Invalid translation id!');

  return translationToString(i);
};
