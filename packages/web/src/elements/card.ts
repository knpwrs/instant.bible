import styled, { ThemedFn } from '../util/styled';

const getBackgroundColor: ThemedFn = ({ theme }) => theme.component.background;

const getFocusBackgroundColor: ThemedFn = ({ theme }) =>
  theme.component.focus.background;

const getFocusBorderColor: ThemedFn = ({ theme }) =>
  theme.component.focus.border;

export default styled('div')`
  border-radius: 10px;
  padding: 15px;
  background: ${getBackgroundColor};
  &:focus {
    outline: none;
    background: ${getFocusBackgroundColor};
    border: 1px solid ${getFocusBorderColor};
  }
`;
