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
  floralWhite: '#FFFCF3', // https://www.wolframalpha.com/input/?i=%23FFFCF3
  pantone135c: '#FFC542', // https://www.wolframalpha.com/input/?i=%23FFC542
  pantone2728c: '#0062FF', // https://www.wolframalpha.com/input/?i=%230062FF
  pantone4332x: '#13131A', // https://www.wolframalpha.com/input/?i=%2313131A
  pantone532: '#1C1C24', // https://www.wolframalpha.com/input/?i=%231c1c24
  pewter: '#92929D', // https://www.wolframalpha.com/input/?i=%2392929D
  theLeague: '#FAFAFB', // https://www.colourlovers.com/color/FAFAFB
  whipLashCream: '#44444F', // https://www.colourlovers.com/color/44444F
  white: '#FFFFFF',
};

export type Font = {
  family: string;
  weight: number;
  size: string;
};

const base = {
  colors,
  text: {
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
  background: colors.theLeague,
  text: {
    ...base.text,
    color: colors.whipLashCream,
  },
  component: {
    background: colors.white,
    focus: {
      background: colors.floralWhite,
      border: colors.pantone135c,
    },
    input: {
      placeholder: colors.pewter,
    },
  },
};

export type Theme = typeof light;

export const dark: Theme = {
  key: 'dark',
  ...base,
  background: colors.pantone4332x,
  text: {
    ...base.text,
    color: colors.theLeague,
  },
  component: {
    background: colors.pantone532,
    focus: {
      background: colors.pantone532,
      border: colors.pantone2728c,
    },
    input: {
      placeholder: colors.pewter,
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

export const fontToCss = (font: Font): ReturnType<typeof css> => css`
  font-family: ${font.family};
  font-weight: ${font.weight};
  font-size: ${font.size};
`;
