import { isNumber } from 'lodash';
import * as proto from '../../proto';
import en from './en';

export const bookToString = (book?: proto.instantbible.data.Book | null) => {
  if (!isNumber(book)) throw new Error('Invalid book!');

  return en[book];
};

export const verseKeyToString = (
  key?: proto.instantbible.data.IVerseKey | null,
) => {
  if (!key) throw new Error('Invalid key!');

  return `${bookToString(key.book)} ${key.chapter}:${key.verse}`;
};

export const verseKeyToObject = (
  key?: proto.instantbible.data.IVerseKey | null,
) => {
  if (!key || !isNumber(key.book) || !isNumber(key.chapter) || !isNumber(key.verse)) {
    throw new Error('Invalid key!');
  }

  return {
    book: key.book,
    chapter: key.chapter,
    verse: key.verse,
  };
};

export const translationToString = (i: number | string) => {
  if (typeof i === 'string') {
    return proto.instantbible.data.Translation[parseInt(i, 10)];
  }

  return proto.instantbible.data.Translation[i];
};

export const textToTranslationsObject = (
  text?: string[] | null,
): { [key in proto.instantbible.data.Translation]: string } => {
  if (!text) throw new Error('Invalid text!');

  return text.reduce(
    (acc, txt, i) => ({
      ...acc,
      [i]: txt,
    }),
    {} as ReturnType<typeof textToTranslationsObject>,
  );
};

export const topTranslation = (i?: number | null) => {
  if (!isNumber(i)) throw new Error('Invalid translation id!');

  return i;
};
