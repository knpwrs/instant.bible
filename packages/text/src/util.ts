import { join } from 'path';
import { promises as fs, existsSync } from 'fs';
import debug from 'debug';
import * as proto from './proto';

const { TranslationData, Translation } = proto.instantbible.data;

const d = debug('util');

export const DATA_DIR = join(__dirname, '../data');

export const serialize = async (translation: string, verses: Array<proto.instantbible.data.VerseText>) => {
  d(`Serializing ${translation}`);
  const data = new TranslationData({
    translation: Translation[translation],
    verses,
  });
  await fs.writeFile(join(DATA_DIR, `${translation}.pb`), TranslationData.encode(data).finish());
};

export const exists = (translation: string) =>
  existsSync(join(DATA_DIR, `${translation}.pb`));
