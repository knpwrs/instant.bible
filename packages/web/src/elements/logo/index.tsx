// Font used is Baloo from https://www.fontsquirrel.com/fonts/baloo
// See `SIL Open Font License.txt`

import * as React from 'react';
import Light from './instantbible-logo-light.svg';
import LightIcon from './instantbible-icon-light.svg';
import Dark from './instantbible-logo-dark.svg';
import DarkAlt from './instantbible-logo-dark-alt.svg';
import DarkIcon from './instantbible-icon-dark.svg';
import { useTheme } from '../../util/theme';

export type Props = {
  alt?: boolean;
  icon?: boolean;
};

const Logo: React.FunctionComponent<Props> = React.memo(({ alt, icon }) => {
  const { key: themeKey } = useTheme();
  const dark = themeKey === 'dark';

  if (dark && icon) {
    return <DarkIcon />;
  }

  if (icon) {
    return <LightIcon />;
  }

  if (dark && alt) {
    return <DarkAlt />;
  }

  if (dark) {
    return <Dark />;
  }

  return <Light />;
});

export default Logo;
