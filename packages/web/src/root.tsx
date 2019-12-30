import * as React from 'react';
import { hot } from 'react-hot-loader/root';
import { I18nProvider } from '@lingui/react';
import { Provider as ReduxProvider } from 'react-redux';
import { ThemeProvider } from './util/theme';
import App from './app';
import store from './state';

const Root: React.FunctionComponent = () => (
  <ReduxProvider store={store}>
    <I18nProvider language="en">
      <ThemeProvider>
        <App />
      </ThemeProvider>
    </I18nProvider>
  </ReduxProvider>
);

export default hot(Root);
