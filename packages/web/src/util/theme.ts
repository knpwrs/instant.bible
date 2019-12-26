export const light = {
  key: 'light' as 'light' | 'dark',
  background: '#FAFAFB',
};

export type Theme = typeof light;

export const dark: Theme = {
  key: 'dark',
  background: '#13131A',
};
