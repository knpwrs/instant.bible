import * as React from 'react';
import { css } from '@emotion/core';
import { Trans } from '@lingui/macro';
import styled, { ThemedFn } from '../util/styled';
import { fontToCss } from '../util/theme';
import { useDirty } from '../state/search';

const getColor: ThemedFn = ({ theme }) => theme.component.input.placeholder;

const getFontStyle: ThemedFn = ({ theme }) =>
  fontToCss(theme.text.subhead4Regular);

const Root = styled('footer')`
  color: ${getColor};
  ${getFontStyle}
  opacity: 0.25;
  text-align: center;
  &:hover {
    opacity: 1;
  }
`;

export default React.memo((props: React.HTMLProps<HTMLParagraphElement>) => {
  const dirty = useDirty();

  return (
    <Root {...props}>
      <p>
        <Trans>Copyright &copy; {new Date().getFullYear()} instant.bible</Trans>
      </p>
      {dirty ? (
        <>
          <p>
            <Trans>
              The Holy Bible, Berean Study Bible, BSB Copyright ©2016, 2018,
              2020 by Bible Hub Used by Permission. All Rights Reserved
              Worldwide.
            </Trans>
          </p>
          <p>
            The Holy Bible, King James Version, KJV is in the public domain and
            not subject to copyright.
          </p>
          <p>
            <Trans>
              The NET Bible®{' '}
              <a
                href="https://netbible.com"
                target="_blank"
                css={css`
                  color: inherit;
                `}
              >
                https://netbible.com
              </a>{' '}
              copyright ©1996, 2019 used with permission from Biblical Studies
              Press, L.L.C. All rights reserved.
            </Trans>
          </p>
        </>
      ) : null}
    </Root>
  );
});
