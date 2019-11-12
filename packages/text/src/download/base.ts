import debug from 'debug';
import { exists, serialize } from '../util';
import * as proto from '../proto';

export type Downloader = ({ d: Debugger }) => Promise<Array<proto.instantbible.data.VerseText>>;

export default async (translation: string, downloader: Downloader) => {
  const d = debug(`download:${translation}`);

  if (exists(translation)) {
    d('Data exists. Skipping download.');

    return;
  }

  const verses = await downloader({ d });

  await serialize(translation, verses);
};
