import * as React from 'react';
import {
  useTheme as useThemeFn,
  ThemeProvider as ThemeProviderComp,
} from 'emotion-theming';
import { Global, css } from '@emotion/core';
import { lighten, mix, transparentize } from 'polished';
import { noop, isString } from 'lodash';

export type ThemeKey = 'light' | 'dark';
const storageKey = 'themeKey';

const colors = {
  blue: '#0062FF',
  gold: '#FFC542',
  gray01: '#92929D',
  gray02: '#44444F',
  gray03: '#1C1C24',
  gray04: '#13131A',
  white01: '#FFFFFF',
  white02: '#F8F8F9',
  black: '#171725',
  green: '#3DD598',
};

export type Font = {
  family: string;
  weight: number;
  size: string;
};

const base = {
  colors,
  text: {
    secondaryColor: '#696974',
    subhead3Regular: {
      family: 'Roboto',
      weight: 400,
      size: '14px',
    },
    subhead3Medium: {
      family: 'Roboto',
      weight: 500,
      size: '14px',
    },
    subhead4Regular: {
      family: 'Roboto',
      weight: 400,
      size: '14px',
    },
  },
};

export const light = {
  key: 'light' as ThemeKey,
  ...base,
  background: colors.white02,
  facade: transparentize(0.7, colors.black),
  text: {
    ...base.text,
    color: colors.gray02,
    highlightColor: colors.blue,
  },
  component: {
    background: colors.white01,
    focus: {
      border: colors.blue,
      background: lighten(0.48, colors.blue),
    },
    input: {
      placeholder: colors.gray01,
    },
    progress: {
      background: '#E2E2EA',
      foreground: colors.green,
    },
    checkbox: {
      border: '#B5B5BE',
      checked: colors.green,
    },
    icon: colors.gray01,
  },
};

export type AppTheme = typeof light;

export const dark: AppTheme = {
  key: 'dark',
  ...base,
  background: colors.gray04,
  facade: transparentize(0.7, colors.gray01),
  text: {
    ...base.text,
    color: colors.white02,
    highlightColor: colors.gold,
  },
  component: {
    background: colors.gray03,
    focus: {
      border: colors.gold,
      background: mix(0.98, colors.gray03, colors.gold),
    },
    input: {
      placeholder: colors.gray01,
    },
    progress: {
      background: '#CFCFD7',
      foreground: colors.green,
    },
    checkbox: {
      border: '#B5B5BE',
      checked: colors.green,
    },
    icon: colors.gray01,
  },
};

export const useTheme = useThemeFn as () => AppTheme;

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

    if (darkQuery.addEventListener) {
      darkQuery.addEventListener('change', onChange);
    } else if (darkQuery.addListener) {
      darkQuery.addListener(onChange);
    }

    return (): void => {
      if (darkQuery.removeEventListener) {
        darkQuery.removeEventListener('change', onChange);
      } else if (darkQuery.removeListener) {
        darkQuery.removeListener(onChange);
      }
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
          // Only show focus outline when focus was triggered using the keyboard
          :focus:not(:focus-visible) {
            outline: none;
          }

          body {
            background: ${theme.background};
          }
        `}
      />
      {children}
    </ThemeProviderComp>
  );
};

export const fontToCss = (font: Font): ReturnType<typeof css> => css`
  font-family: ${font.family};
  font-weight: ${font.weight};
  font-size: ${font.size};
`;
