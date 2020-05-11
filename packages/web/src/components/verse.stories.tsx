import * as React from 'react';
import { storiesOf } from '@storybook/react';
import { text } from '@storybook/addon-knobs';
import Verse from './verse';
import data from './__mocks__/verse-data';
import * as proto from '../proto';

storiesOf('components/verse', module).add('basic', () => {
  const highlight = text('highlight', 'love').split(' ');

  return (
    <Verse
      title="John 3:16"
      data={data}
      topTranslation={proto.instantbible.data.Translation.NET}
      highlight={highlight}
      verseKey={{
        book: proto.instantbible.data.Book.JOHN,
        chapter: 3,
        verse: 16,
      }}
    />
  );
});
