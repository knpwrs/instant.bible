import * as React from 'react';
import { css } from '@emotion/core';
import { Trans } from '@lingui/macro';
import styled from '@emotion/styled';
import { ThemedFn } from '../util/styled';
import { fontToCss } from '../util/theme';
import * as bp from '../util/breakpoints';
import { useDirty } from '../state/search';

const getColor: ThemedFn = ({ theme }) => theme.component.input.placeholder;

const getFontStyle: ThemedFn = ({ theme }) =>
  fontToCss(theme.text.subhead4Regular);

const Root = styled('p')`
  color: ${getColor};
  ${getFontStyle}
  text-align: center;
  margin: 0 24px;
  max-width: ${bp.lg};
`;

export default React.memo((props: React.HTMLProps<HTMLParagraphElement>) => {
  const dirty = useDirty();

  return (
    <Root {...props}>
      <p>
        <Trans>
          Copyright &copy; {new Date().getFullYear()} instant.bible.
        </Trans>{' '}
        <Trans>App Store and the Apple logo are trademarks of Apple Inc.</Trans>{' '}
        <Trans>
          Google Play and the Google Play logo are trademarks of Google LLC.
        </Trans>{' '}
        <a
          href="/privacy.html"
          css={css`
            color: inherit;
            text-decoration: none;

            &:visited {
              color: inherit;
            }

            &:hover {
              text-decoration: underline;
            }
          `}
        >
          <Trans>Privacy Policy.</Trans>
        </a>
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
