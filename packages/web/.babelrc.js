module.exports = {
  presets: [
    [
      '@babel/preset-env',
      { modules: process.env.NODE_ENV === 'test' && 'commonjs' },
    ],
    '@babel/preset-typescript',
    '@babel/preset-react',
    '@emotion/babel-preset-css-prop',
  ],
  plugins: [
    '@babel/plugin-proposal-optional-chaining',
    'react-hot-loader/babel',
    'emotion',
    'macros',
  ],
};
