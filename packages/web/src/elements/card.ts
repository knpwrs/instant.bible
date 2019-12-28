import styled, { ThemedFn } from '../util/styled';

const getBackgroundColor: ThemedFn = ({ theme }) => theme.component.background;

const getFocusBorderColor: ThemedFn = ({ theme }) =>
  theme.component.focus.border;

export default styled('div')`
  border: 1px solid transparent;
  border-radius: 10px;
  padding: 15px;
  background: ${getBackgroundColor};
  &:focus {
    outline: none;
    border: 1px solid ${getFocusBorderColor};
  }
`;
