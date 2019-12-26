import { css } from '@emotion/core';
import styled, { ThemedFn } from '../util/styled';
import { fontToCss } from '../util/theme';

const getTextColor: ThemedFn = ({ theme }) => theme.text.color;
const getBackgroundColor: ThemedFn = ({ theme }) => theme.component.background;
const getFocusBorderColor: ThemedFn = ({ theme }) =>
  theme.component.focus.border;

const getFontStyle: ThemedFn = ({ theme }) =>
  fontToCss(theme.text.subhead3Medium);

const getPlaceholderStyle: ThemedFn = ({ theme }) => css`
  color: ${theme.component.input.placeholder};
  ${fontToCss(theme.text.subhead3Regular)}
`;

export default styled('input')`
  border: 1px solid transparent;
  border-radius: 10px;
  padding: 11px 12px;
  color: ${getTextColor};
  background: ${getBackgroundColor};

  ${getFontStyle}

  &:focus {
    outline: none;
    border: 1px solid ${getFocusBorderColor};
  }

  &::placeholder {
    ${getPlaceholderStyle}
  }
`;
