import styled, { ThemedFn } from '../util/styled';

const getColor: ThemedFn = ({ theme }) => theme.text.color;

export const Text = styled('span')`
  font-family: Roboto;
  color: ${getColor};
`;

export const Body3 = styled(Text.withComponent('p'))`
  font-size: 14px;
`;

export const Body3Highlight = styled(Body3.withComponent('mark'))`
  font-weight: 700;
  background: none;
`;

export const H5 = styled('h5')`
  font-family: Poppins;
  color: ${getColor};
  font-weight: 600;
  font-size: 18px;
`;

export const Subhead3Medium = styled(Text)`
  font-size: 14px;
  font-weight: 500;
`;
