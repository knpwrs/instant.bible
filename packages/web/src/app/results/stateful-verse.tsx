import * as React from 'react';
import Verse from '../../components/verse';
import { ResType } from '../../state/search';

export type Props = {
  data: ResType['results'][number];
  className?: string;
};

export default React.memo(({ data, ...rest }: Props) => {
  const [selectedKey, setSelectedKey] = React.useState(data.topTranslation);

  return (
    <Verse
      title={data.key}
      data={data.text}
      selectedKey={selectedKey}
      highlight={[]}
      onSelectKey={setSelectedKey}
      {...rest}
    />
  );
});
