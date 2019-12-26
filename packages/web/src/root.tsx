import * as React from 'react';
import { hot } from 'react-hot-loader/root';
import Header from './elements/header';
import Logo from './elements/logo';
import { ThemeProvider } from './util/theme';

const App: React.FunctionComponent = () => (
  <ThemeProvider>
    <Logo />
    <Header>Hello, World!</Header>
  </ThemeProvider>
);

export default hot(App);
