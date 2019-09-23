import debug, { Debugger } from 'debug';
import { Verse, exists, serialize } from '../util';

export type Downloader = ({ d: Debugger }) => Promise<Array<Verse>>;

export default async (translation: string, downloader: Downloader) => {
  const d = debug(`download:${translation}`);

  if (exists(translation)) {
    d('Data exists. Skipping download.');

    return;
  }

  const verses = await downloader({ d });

  await serialize(translation, verses);
};
