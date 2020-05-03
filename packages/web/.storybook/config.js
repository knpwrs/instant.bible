import * as React from 'react';
import { withKnobs, optionsKnob } from '@storybook/addon-knobs';
import { configure, addDecorator } from '@storybook/react';
import { I18nProvider } from '@lingui/react';
import { ThemeProvider } from 'emotion-theming';
import styled from '../src/util/styled';
import * as themeUtil from '../src/util/theme';

const Wrapper = styled.div`
  display: flex;
  flex-direction: column;
  height: 100vh;
`;

const ThemeWrapper = styled.div`
  display: flex;
  flex: 1;
  background: ${({ theme }) => theme.background};
  justify-content: center;
  align-items: center;
`;

const ThemeDecorator = (storyFn, context) => {
  const themes = optionsKnob(
    'Themes',
    { Light: 'light', Dark: 'dark' },
    ['light', 'dark'],
    { display: 'inline-check' },
  )
    .sort()
    .reverse();

  return (
    <Wrapper>
      {themes.map(v => {
        const theme = themeUtil[v];

        return (
          <ThemeProvider key={theme.key} theme={theme}>
            <ThemeWrapper>{storyFn()}</ThemeWrapper>
          </ThemeProvider>
        );
      })}
    </Wrapper>
  );
};

const LinguiDecorator = (storyFn, context) => {
  return <I18nProvider language="en">{storyFn()}</I18nProvider>;
};

addDecorator(ThemeDecorator);
addDecorator(LinguiDecorator);
addDecorator(withKnobs);

const req = require.context('../src', true, /.stories.tsx?$/);
function loadStories() {
  req.keys().forEach(filename => req(filename));
}

configure(loadStories, module);
