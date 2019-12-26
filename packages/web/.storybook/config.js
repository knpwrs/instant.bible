import * as React from 'react';
import { withKnobs } from '@storybook/addon-knobs';
import { configure, addDecorator } from '@storybook/react';
import { ThemeProvider } from 'emotion-theming';
import styled from '../src/util/styled';
import { light, dark } from '../src/util/theme';

const Wrapper = styled.div`
  display: flex;
  flex-direction: row;
`;

const ThemeWrapper = styled.div`
  display: flex;
  width: 50vw;
  height: 100vh;
  padding: 30px;
  background: ${({ theme }) => theme.background};
  justify-content: center;
  align-items: center;
`;

const ThemeDecorator = (storyFn, context) => (
  <Wrapper>
    {[light, dark].map(theme => (
      <ThemeProvider key={theme.key} theme={theme}>
        <ThemeWrapper>{storyFn()}</ThemeWrapper>
      </ThemeProvider>
    ))}
  </Wrapper>
);

addDecorator(ThemeDecorator);
addDecorator(withKnobs);

const req = require.context('../src', true, /.stories.tsx?$/);
function loadStories() {
  req.keys().forEach(filename => req(filename));
}

configure(loadStories, module);
