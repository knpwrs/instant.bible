module.exports = {
  presets: [
    [
      '@babel/preset-env',
      { modules: process.env.NODE_ENV === 'test' && 'commonjs' },
    ],
    '@babel/preset-typescript',
    '@babel/preset-react',
  ],
  plugins: ['react-hot-loader/babel', 'emotion'],
};
