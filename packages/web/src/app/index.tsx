import * as React from 'react';
import { css } from '@emotion/core';
import Header from './header';
import Results from './results';
import Footer from './footer';
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
      <Footer
        css={
          dirty
            ? null
            : css`
                position: absolute;
                bottom: 0;
              `
        }
      />
      <Settings />
    </div>
  );
});
