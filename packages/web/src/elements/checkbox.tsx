import * as React from 'react';
import { css } from '@emotion/core';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faCheck } from '@fortawesome/free-solid-svg-icons';
import { Body2 } from './typography';
import styled, { ThemedFn } from '../util/styled';

const RootLabel = styled('label')`
  display: flex;
  flex-direction: row;
  align-items: center;
  width: 100%;
  height: 26px;
`;

type BoxProps = {
  checked: boolean;
  disabled?: boolean;
};

const getBoxColor: ThemedFn<BoxProps> = ({
  theme,
  checked,
  disabled,
}): string => {
  if (!checked) {
    return 'none';
  }

  if (checked && disabled) {
    return theme.component.checkbox.border;
  }

  if (checked) {
    return theme.component.checkbox.checked;
  }

  return 'none';
};

const getBoxBorderColor: ThemedFn<BoxProps> = ({
  theme,
  checked,
  disabled,
}): string => {
  if (disabled || !checked) {
    return theme.component.checkbox.border;
  }

  return theme.component.checkbox.checked;
};

const Box = styled('div')<BoxProps>`
  position: relative;
  width: 16px;
  height: 16px;
  margin: 0 14px;
  border-radius: 4px;
  font-size: 12px;
  border: 1px solid ${getBoxBorderColor};
  background: ${getBoxColor};
`;

export type Props = React.PropsWithChildren<{
  value: boolean;
  onChange: (val: boolean) => unknown;
  disabled?: boolean;
}>;

export default React.memo(({ children, onChange, disabled, value }: Props) => {
  const maybeHandleClick = disabled ? undefined : () => onChange(!value);

  return (
    <RootLabel onClick={maybeHandleClick} role="checkbox">
      <Box checked={value} disabled={disabled}>
        <input
          type="checkbox"
          checked={value}
          readOnly={true}
          css={css`
            opacity: 0;
          `}
        />
        {value ? (
          <FontAwesomeIcon
            icon={faCheck}
            color="#FFF"
            size="sm"
            css={css`
              position: absolute;
              top: 2px;
              left: 2px;
            `}
          />
        ) : null}
      </Box>
      <Body2
        css={css`
          width: calc(100% - 44px);
        `}
      >
        {children}
      </Body2>
    </RootLabel>
  );
});
