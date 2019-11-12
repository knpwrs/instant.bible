import * as got from 'got';
import { books } from '../meta';
import base, { Downloader } from './base';
import * as proto from '../proto';

const { VerseText } = proto.instantbible.data;

const makeUrl = (book: string, chapter: number) => {
  const passage = `${book} ${chapter}`;

  return `http://labs.bible.org/api/?passage=${encodeURIComponent(
    passage,
  )}&type=json&formatting=plain`;
};

type ResponseVerse = {
  book: proto.instantbible.data.Book;
  chapter: string;
  verse: string;
  text: string;
};

const download: Downloader = async ({ d }) => {
  const data: Array<ResponseVerse> = [];

  for (const { name, chapters, proto } of books) {
    for (let chapter = 1; chapter <= chapters; chapter++) {
      d(`Downloading ${name} ${chapter}`);

      const { body } = (await got(makeUrl(name, chapter), {
        json: true,
      })) as { body: Array<ResponseVerse> };

      data.push(...body.map(b => ({ ...b, book: proto })));
    }
  }

  const verses: Array<proto.instantbible.data.VerseText> = data.map(v => (new VerseText({
    key: {
      book: v.book,
      chapter: parseInt(v.chapter, 10),
      verse: parseInt(v.verse, 10),
    },
    text: v.text,
  })));

  return verses;
};

base('NET', download);
