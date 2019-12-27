import * as React from 'react';
import { Trans } from '@lingui/macro';
import styled, { ThemedFn } from '../util/styled';
import { fontToCss } from '../util/theme';

const getColor: ThemedFn = ({ theme }) => theme.component.input.placeholder;

const getFontStyle: ThemedFn = ({ theme }) =>
  fontToCss(theme.text.subhead4Regular);

const Root = styled('p')`
  color: ${getColor};
  ${getFontStyle}
  opacity: 0.25;
  &:hover {
    opacity: 0.4;
  }
`;

export default React.memo((props: React.HTMLProps<HTMLParagraphElement>) => (
  <Root {...props}>
    <Trans id="copyright">
      Copyright &copy; {new Date().getFullYear()} instant.bible
    </Trans>
  </Root>
));
