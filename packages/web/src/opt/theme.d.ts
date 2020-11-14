import '@emotion/react';
import { AppTheme } from '../util/theme';

declare module '@emotion/react' {
  // eslint-disable-next-line @typescript-eslint/no-empty-interface
  export interface Theme extends AppTheme {}
}
