import styled from '../util/styled';

export default styled('h1')`
  color: ${({ theme }): string => theme.text.color};
  font-family: Roboto;
`;
