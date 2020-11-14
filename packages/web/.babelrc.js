module.exports = {
  presets: [
    [
      '@babel/preset-env',
      { modules: process.env.NODE_ENV === 'test' && 'commonjs' },
    ],
    '@babel/preset-typescript',
    [
      '@babel/preset-react',
      { runtime: 'automatic', importSource: '@emotion/react' },
    ],
  ],
  plugins: [
    '@emotion',
    '@babel/plugin-proposal-optional-chaining',
    'react-hot-loader/babel',
    'babel-plugin-lodash',
    'macros',
  ],
};
