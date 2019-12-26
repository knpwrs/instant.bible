import * as React from 'react';
import {
  useTheme as useThemeFn,
  ThemeProvider as ThemeProviderComp,
} from 'emotion-theming';
import { Global, css } from '@emotion/core';
import { noop, isString } from 'lodash';

export type ThemeKey = 'light' | 'dark';
const storageKey = 'themeKey';

const colors = {
  // Named colors
  whipLashCream: '#44444F', // https://www.colourlovers.com/color/44444F
  theLeague: '#FAFAFB', // https://www.colourlovers.com/color/FAFAFB
  pantone4332x: '#13131A', // https://www.wolframalpha.com/input/?i=%2313131A
  // Other colors
  blue: '#0062FF',
  darkBlue: '#1C1C24',
  paleYellow: '#FFFCF3',
  white: '#FFFFFF',
  yellow: '#FFC542',
};

const base = {
  colors,
};

export const light = {
  key: 'light' as ThemeKey,
  ...base,
  background: colors.theLeague,
  text: colors.whipLashCream,
  component: {
    background: colors.white,
    focus: {
      background: colors.paleYellow,
      border: colors.yellow,
    },
  },
};

export type Theme = typeof light;

export const dark: Theme = {
  key: 'dark',
  ...base,
  background: colors.pantone4332x,
  text: colors.theLeague,
  component: {
    background: colors.darkBlue,
    focus: {
      background: colors.darkBlue,
      border: colors.blue,
    },
  },
};

export const useTheme = useThemeFn as () => Theme;

const darkQuery = window.matchMedia('(prefers-color-scheme: dark)');
const defaultKey = darkQuery.matches ? 'dark' : 'light';

export type ThemeSettingObj = {
  key: ThemeKey;
  setLight: () => void;
  setDark: () => void;
  clear: () => void;
};

export const useThemeSetting = (): ThemeSettingObj => {
  const [key, setKey] = React.useState<ThemeKey>(defaultKey);

  React.useEffect(() => {
    const saved = window.localStorage.getItem(storageKey);

    if (isString(saved)) {
      setKey(saved as ThemeKey);

      return noop;
    }

    const onChange = ({ matches }: MediaQueryListEvent): void => {
      setKey(matches ? 'dark' : 'light');
    };

    darkQuery.addEventListener('change', onChange);

    return (): void => {
      darkQuery.removeEventListener('change', onChange);
    };
  }, [setKey]);

  const setLight = React.useCallback(() => {
    setKey('light');
    window.localStorage.setItem(storageKey, 'light');
  }, [setKey]);

  const setDark = React.useCallback(() => {
    setKey('dark');
    window.localStorage.setItem(storageKey, 'dark');
  }, [setKey]);

  const clear = React.useCallback(() => {
    setKey(defaultKey);
    window.localStorage.removeItem(storageKey);
  }, []);

  return { key, setLight, setDark, clear };
};

export const ThemeProvider: React.FunctionComponent<{
  children: React.ReactNode;
}> = ({ children }) => {
  const { key } = useThemeSetting();
  const theme = key === 'dark' ? dark : light;

  return (
    <ThemeProviderComp theme={theme}>
      <Global
        styles={css`
          body {
            background: ${theme.background};
          }
        `}
      />
      {children}
    </ThemeProviderComp>
  );
};
