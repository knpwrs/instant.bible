import * as React from 'react';
import { css } from '@emotion/core';
import Verse from './stateful-verse';
import { useRestedQuery } from '../../state/search';
import { useQuery } from '../../util/history';

export default React.memo(() => {
  const query = useRestedQuery(useQuery().q);

  return (
    <div
      css={css`
        margin-top: 70px;
      `}
    >
      {query?.res?.results.map(q => (
        <Verse
          key={q.key}
          data={q}
          css={css`
            margin-bottom: 15px;
          `}
        />
      ))}
    </div>
  );
});
