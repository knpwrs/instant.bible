import * as got from 'got';
import shaFn = require('crypto-js/sha256');
import { keyBy } from 'lodash';
import base, { Downloader } from './base';
import { books } from '../meta';
import * as proto from '../proto';

const { VerseText } = proto.instantbible.data;

const URL = 'https://bereanbible.com/bsb.txt';
const SHA256 =
  'a83b94fa1ddc659eefff1c7b0a81706a2b06ee2f6c6b77654c5a30ad1a3b8c86';

const verseLine = /^(.+) (\d+):(\d+)\t(.+)$/;

const booksMap = keyBy(books, 'name');

const mapBookName = (book: string) => {
  if (book === 'Psalm') {
    return 'Psalms';
  }

  return book;
};

const download: Downloader = async ({ d }) => {
  d('Downloading text');

  const { body: rawText } = await got(URL);
  const rawTextSha = shaFn(rawText).toString();

  if (rawTextSha !== SHA256) {
    throw new Error(
      `SHA of BSB text does not match! Expected ${SHA256} but got ${rawTextSha}`,
    );
  }

  const lines = (rawText as string).split(/\r?\n/);
  const verses: Array<proto.instantbible.data.VerseText> = [];

  for (const line of lines) {
    const match = line.match(verseLine);
    if (!match) continue;
    const [, book, chapter, verse, text] = match;
    try {
      verses.push(
        new VerseText({
          key: {
            book: booksMap[mapBookName(book)].proto,
            chapter: parseInt(chapter, 10),
            verse: parseInt(verse, 10),
          },
          text,
        }),
      );
    } catch (e) {
      d(e);
      d({ book, chapter, verse, text });
      process.exit(-1);
    }
  }

  return verses;
};

base('BSB', download);
