import { join } from 'path';
import { promises as fs, existsSync } from 'fs';
import debug from 'debug';

const d = debug('util');

export const DATA_DIR = join(__dirname, '../data');

export type Verse = {
  book: string;
  chapter: number;
  verse: number;
  text: string;
};

export const serialize = async (translation: string, verses: Array<Verse>) => {
  d(`Serializing ${translation}`);
  const str = JSON.stringify(verses, null, 2);
  await fs.writeFile(join(DATA_DIR, `${translation}.json`), str);
};

export const exists = (translation: string) =>
  existsSync(join(DATA_DIR, `${translation}.json`));
