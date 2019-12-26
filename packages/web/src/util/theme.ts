import { useTheme as useThemeFn } from 'emotion-theming';

export const light = {
  key: 'light' as 'light' | 'dark',
  background: '#FAFAFB',
};

export type Theme = typeof light;

export const dark: Theme = {
  key: 'dark',
  background: '#13131A',
};

export const useTheme = useThemeFn as () => Theme;
