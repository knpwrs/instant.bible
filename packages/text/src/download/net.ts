import * as got from 'got';
import debug from 'debug';
import { books } from '../meta';
import { Verse, serialize } from '../util';

const abbrev = 'NET';
const d = debug(`download:${abbrev}`);

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

export default async () => {
  const data: Array<ResponseVerse> = [];

  for (const { name, chapters } of books) {
    for (let chapter = 1; chapter <= chapters; chapter++) {
      d(`Downloading ${name} ${chapter}`);

      const { body } = (await got(makeUrl(name, chapter), {
        json: true,
      })) as { body: Array<ResponseVerse> };

      data.push(...body);
    }
  }

  const verses: Array<Verse> = data.map(v => ({
    book: v.bookname,
    chapter: parseInt(v.chapter, 10),
    verse: parseInt(v.verse, 10),
    text: v.text,
  }));

  await serialize(abbrev, verses);
};
