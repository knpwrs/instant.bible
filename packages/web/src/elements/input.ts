import { css } from '@emotion/core';
import styled, { ThemedFn } from '../util/styled';

const getTextColor: ThemedFn = ({ theme }) => theme.text.color;
const getBackgroundColor: ThemedFn = ({ theme }) => theme.component.background;
const getFocusBorderColor: ThemedFn = ({ theme }) =>
  theme.component.focus.border;

const getFontStyle: ThemedFn = ({ theme }) => css`
  font-family: ${theme.text.subhead3Medium.family};
  font-weight: ${theme.text.subhead3Medium.weight};
  font-size: ${theme.text.subhead3Medium.size};
`;

const getPlaceholderStyle: ThemedFn = ({ theme }) => css`
  color: ${theme.component.input.placeholder};
  font-family: ${theme.text.subhead3Regular.family};
  font-weight: ${theme.text.subhead3Regular.weight};
  font-size: ${theme.text.subhead3Regular.size};
`;

export default styled('input')`
  border: none;
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
