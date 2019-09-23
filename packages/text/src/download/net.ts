import * as got from 'got';
import { books } from '../meta';
import base, { Downloader } from './base';
import { Verse } from '../util';

const makeUrl = (book: string, chapter: number) => {
  const passage = `${book} ${chapter}`;

  return `http://labs.bible.org/api/?passage=${encodeURIComponent(
    passage,
  )}&type=json`;
};

type ResponseVerse = {
  bookname: string;
  chapter: string;
  verse: string;
  text: string;
};

const download: Downloader = async ({ d }) => {
  const data: Array<ResponseVerse> = [];

  for (const { name, chapters } of books) {
    for (let chapter = 1; chapter <= chapters; chapter++) {
      d(`Downloading ${name} ${chapter}`);

      const { body } = (await got(makeUrl(name, chapter), {
        json: true,
      })) as { body: Array<ResponseVerse> };

      data.push(...body.map(b => ({ ...b, bookname: name })));
    }
  }

  const verses: Array<Verse> = data.map(v => ({
    book: v.bookname,
    chapter: parseInt(v.chapter, 10),
    verse: parseInt(v.verse, 10),
    text: v.text,
  }));

  return verses;
};

base('NET', download);
