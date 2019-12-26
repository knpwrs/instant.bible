const path = require('path');

module.exports = ({ config }) => {
  config.entry.unshift('typeface-roboto');
  config.entry.unshift('sanitize.css');

  const fileLoaderRule = config.module.rules.find(rule =>
    rule.test.test('.svg'),
  );
  fileLoaderRule.exclude = /\.svg$/;

  config.module.rules.unshift(
    {
      test: /\.tsx?$/,
      exclude: /node_modules/,
      loader: require.resolve('babel-loader'),
    },
    {
      test: /\.svg$/,
      loader: require.resolve('@svgr/webpack'),
    },
  );
  config.resolve.extensions.unshift('.ts', '.tsx');

  return config;
};
