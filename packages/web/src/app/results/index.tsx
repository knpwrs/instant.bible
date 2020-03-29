import * as React from 'react';
import { css } from '@emotion/core';
import Verse from './stateful-verse';
import { useResults } from '../../state/search';

export default React.memo(() => {
  const query = useResults();

  return (
    <div
      css={css`
        margin-top: 70px;
      `}
    >
      {query?.res.map(q => (
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
