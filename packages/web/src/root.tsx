import * as React from 'react';
import { hot } from 'react-hot-loader/root';
import { I18nProvider } from '@lingui/react';
import { ThemeProvider } from './util/theme';
import App from './app';

const Root: React.FunctionComponent = () => (
  <ThemeProvider>
    <I18nProvider language="en">
      <App />
    </I18nProvider>
  </ThemeProvider>
);

export default hot(Root);
