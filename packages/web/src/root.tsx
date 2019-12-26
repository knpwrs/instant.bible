import * as React from 'react';
import { hot } from 'react-hot-loader/root';
import { ThemeProvider } from './util/theme';
import App from './app';

const Root: React.FunctionComponent = () => (
  <ThemeProvider>
    <App />
  </ThemeProvider>
);

export default hot(Root);
