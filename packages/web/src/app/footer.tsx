import * as React from 'react';
import { css } from '@emotion/core';
import Copyright from './copyright';
import styled from '../util/styled';
import apple1x from '../images/apple.png';
import apple2x from '../images/apple@2x.png';
import apple3x from '../images/apple@3x.png';
import google1x from '../images/google.png';
import google2x from '../images/google@2x.png';
import google3x from '../images/google@3x.png';

const Root = styled('footer')`
  opacity: 0.5;
  &:hover {
    opacity: 1;
  }
`;

export default React.memo((props: React.HTMLProps<HTMLDivElement>) => {
  return (
    <Root {...props}>
      <div
        css={css`
          display: flex;
          justify-content: center;
        `}
      >
        {[
          [
            'https://apps.apple.com/us/app/id1533722003',
            apple1x,
            apple2x,
            apple3x,
          ] as const,
          [
            'https://play.google.com/store/apps/details?id=bible.instant',
            google1x,
            google2x,
            google3x,
          ] as const,
        ].map(([link, one, two, three]) => (
          <a href={link} key={link} target="_blank">
            <img
              css={css`
                height: 40px;
                margin-right: 12px;
                border: none;
              `}
              src={one}
              srcSet={`${one} 1x, ${two} 2x, ${three} 3x`}
            />
          </a>
        ))}
      </div>
      <Copyright />
    </Root>
  );
});
