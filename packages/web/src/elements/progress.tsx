import * as React from 'react';
import { css } from '@emotion/core';
import { Text } from './typography';
import styled from '../util/styled';

const Root = styled('div')`
  display: flex;
  flex-direction: row;
  align-items: center;
  width: 100%;
  height: 16px;
`;

const Bar = styled('div')<{ isFill?: boolean }>`
  width: calc(100% - 44px);
  height: 8px;
  border-radius: 4px;
  background: ${({ theme, isFill: fill }) =>
    fill
      ? theme.component.progress.foreground
      : theme.component.progress.background};
  position: ${({ isFill: fill }) => (fill ? 'absolute' : 'relative')};
  left: 0;
  top: 0;
`;

export type Props = {
  value: number;
};

export default React.memo(({ value }: Props) => (
  <Root>
    <Text
      secondary={true}
      css={css`
        width: 44px;
        font-size: 14px;
      `}
    >
      {(value * 100).toFixed(0)}%
    </Text>
    <Bar>
      <Bar isFill={true} style={{ width: `${(value * 100).toFixed(0)}%` }} />
    </Bar>
  </Root>
));
