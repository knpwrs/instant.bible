const path = require('path');

module.exports = ({ config }) => {
  config.entry.unshift('typeface-roboto');
  config.entry.unshift('sanitize.css');
  config.module.rules.push({
    test: /\.tsx?$/,
    exclude: /node_modules/,
    loader: require.resolve('babel-loader'),
  });
  config.resolve.extensions.unshift('.ts', '.tsx');

  return config;
};
