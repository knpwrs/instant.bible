import * as React from 'react';
import { I18nProvider } from '@lingui/react';
import { ThemeProvider } from 'emotion-theming';
import { render } from '@testing-library/react';
import { light } from '../util/theme';

export default (children: React.ReactNode) =>
  render(
    <I18nProvider language="en">
      <ThemeProvider theme={light}>{children}</ThemeProvider>
    </I18nProvider>,
  );
