import styled, { ThemedFn } from '../util/styled';

const getColor: ThemedFn<{ secondary?: boolean }> = ({ theme, secondary }) =>
  secondary ? theme.text.secondaryColor : theme.text.color;

export const Text = styled('span')<{ secondary?: boolean }>`
  font-family: Roboto;
  color: ${getColor};
`;

export const Body2 = styled(Text)`
  font-size: 16px;
`;

export const Body3 = styled(Text.withComponent('p'))`
  font-size: 14px;
`;

const getHighlightColor: ThemedFn = ({ theme }) => theme.text.highlightColor;

export const Body3Highlight = styled(Body3.withComponent('mark'))`
  font-weight: 700;
  background: none;
  color: ${getHighlightColor};
`;

export const H4 = styled('h4')`
  font-family: Poppins;
  color: ${getColor};
  font-weight: 600;
  font-size: 24px;
  margin-top: 0;
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
