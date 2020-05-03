import * as React from 'react';
import { hot } from 'react-hot-loader/root';
import { setupI18n } from '@lingui/core';
import { I18nProvider } from '@lingui/react';
import { Provider as ReduxProvider } from 'react-redux';
import { ThemeProvider } from './util/theme';
import App from './app';
import store from './state';

// @ts-ignore: message catalog is untyped but generated
import en from './locales/en/messages';

const i18n = setupI18n();
i18n.load({ en });
i18n.activate('en');

const Root: React.FunctionComponent = () => (
  <ReduxProvider store={store}>
    <I18nProvider language="en" i18n={i18n}>
      <ThemeProvider>
        <App />
      </ThemeProvider>
    </I18nProvider>
  </ReduxProvider>
);

export default hot(Root);
