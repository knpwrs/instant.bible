import * as React from 'react';
import { css } from '@emotion/core';
import { Logo, Input } from '../elements';
import Copyright from './copyright';

export default React.memo(() => {
  return (
    <div
      css={css`
        width: 100vw;
        height: 100vh;
        display: flex;
        justify-content: center;
        align-items: center;
        flex-direction: column;
      `}
    >
      <div
        css={css`
          width: 40%;
          display: flex;
          flex-direction: column;
          align-items: center;
        `}
      >
        <div
          css={css`
            width: 50%;
          `}
        >
          <Logo />
        </div>
        <Input
          css={css`
            width: 100%;
            margin-top: 30px;
          `}
          placeholder="Search..."
        />
      </div>
      <Copyright
        css={css`
          position: absolute;
          bottom: 0;
        `}
      />
    </div>
  );
});
