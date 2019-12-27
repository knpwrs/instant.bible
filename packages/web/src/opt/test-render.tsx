import * as React from 'react';
import { ThemeProvider } from 'emotion-theming';
import { render } from '@testing-library/react';
import { light } from '../util/theme';

export default (children: React.ReactNode): ReturnType<typeof render> =>
  render(<ThemeProvider theme={light}>{children}</ThemeProvider>);
