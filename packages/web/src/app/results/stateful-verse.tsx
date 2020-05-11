import * as React from 'react';
import Verse from '../../components/verse';
import { SliceState as SearchState, useVerse } from '../../state/search';

export type Props = {
  data: SearchState['queries'][string]['res'][number];
  className?: string;
};

export default React.memo(({ data, ...rest }: Props) => {
  const verse = useVerse(data.id);

  return (
    <Verse
      title={data.id}
      data={verse}
      topTranslation={data.topTranslation}
      highlight={data.highlights}
      verseKey={data.key}
      {...rest}
    />
  );
});
