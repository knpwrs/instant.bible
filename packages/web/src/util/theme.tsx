import * as React from 'react';
import {
  useTheme as useThemeFn,
  ThemeProvider as ThemeProviderComp,
} from 'emotion-theming';
import { Global, css } from '@emotion/core';
import { noop, isString } from 'lodash';

export type ThemeKey = 'light' | 'dark';
const storageKey = 'themeKey';

export const light = {
  key: 'light' as ThemeKey,
  background: '#FAFAFB',
  text: '#44444F',
};

export type Theme = typeof light;

export const dark: Theme = {
  key: 'dark',
  background: '#13131A',
  text: '#FAFAFB',
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
