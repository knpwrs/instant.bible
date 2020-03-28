import * as React from 'react';
import { css } from '@emotion/core';
import Header from './header';
import Results from './results';
import Copyright from './copyright';
import Settings from './settings';
import { useDirty } from '../state/search';

export default React.memo(() => {
  const dirty = useDirty();

  return (
    <div
      css={css`
        position: relative;
        width: 100vw;
        min-height: 100vh;
        padding: 0 15px 35px 15px;
        display: flex;
        justify-content: ${dirty ? 'flex-start' : 'center'};
        align-items: center;
        flex-direction: column;
      `}
    >
      <Header />
      {dirty ? <Results /> : null}
      <Copyright
        css={css`
          position: absolute;
          bottom: 0;
        `}
      />
      <Settings />
    </div>
  );
});
