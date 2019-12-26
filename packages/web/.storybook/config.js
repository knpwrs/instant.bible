import * as React from 'react';
import { withKnobs } from '@storybook/addon-knobs';
import { configure, addDecorator } from '@storybook/react';
import styled from '@emotion/styled';
import { ThemeProvider } from 'emotion-theming';

const theme = {};

const Wrapper = styled.div`
  padding: 30px;
`;

const ThemeDecorator = (storyFn, context) => (
  <Wrapper>
    <ThemeProvider theme={theme}>{storyFn()}</ThemeProvider>
  </Wrapper>
);

addDecorator(ThemeDecorator);
addDecorator(withKnobs);

const req = require.context('../src', true, /.stories.tsx?$/);
function loadStories() {
  req.keys().forEach(filename => req(filename));
}

configure(loadStories, module);
