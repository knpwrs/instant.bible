import styled from '@emotion/styled';

export default styled.div`
  border: 1px solid transparent;
  border-radius: 10px;
  padding: 15px;
  background: ${({ theme }) => theme.component.background};
  &:focus {
    outline: none;
    background: ${({ theme }) => theme.component.focus.background};
    border: 1px solid ${({ theme }) => theme.component.focus.border};
  }
`;
