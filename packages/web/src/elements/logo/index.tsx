// Font used is Baloo from https://www.fontsquirrel.com/fonts/baloo
// See `SIL Open Font License.txt`

import * as React from 'react';
import { useTheme } from 'emotion-theming';
import Light from './instantbible-logo-light.svg';
import Dark from './instantbible-logo-dark.svg';
import DarkAlt from './instantbible-logo-dark-alt.svg';
import { Theme } from '../../util/theme';

export type Props = {
  alt?: boolean;
};

const Logo: React.FunctionComponent<Props> = React.memo(({ alt }) => {
  const { key: themeKey } = useTheme() as Theme;

  if (themeKey === 'dark' && alt) {
    return <DarkAlt />;
  }

  if (themeKey === 'dark') {
    return <Dark />;
  }

  return <Light />;
});

export default Logo;
