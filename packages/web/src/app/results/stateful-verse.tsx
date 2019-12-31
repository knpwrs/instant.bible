import * as React from 'react';
import Verse from '../../components/verse';
import { SliceState as SearchState, useVerse } from '../../state/search';

export type Props = {
  data: SearchState['queries'][string]['res'][number];
  className?: string;
};

export default React.memo(({ data, ...rest }: Props) => {
  const [selectedKey, setSelectedKey] = React.useState(data.topTranslation);
  const verse = useVerse(data.key);

  return (
    <Verse
      title={data.key}
      data={verse}
      selectedKey={selectedKey}
      highlight={data.highlights}
      onSelectKey={setSelectedKey}
      {...rest}
    />
  );
});
