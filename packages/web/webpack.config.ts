import * as webpack from 'webpack';
import { resolve } from 'path';
import * as HtmlWebpackPlugin from 'html-webpack-plugin';

const conf: webpack.Configuration = {
  context: __dirname,
  entry: ['react-hot-loader/patch', 'sanitize.css', 'typeface-roboto', './src'],
  output: {
    path: resolve(__dirname, './dist'),
    filename: 'app.bundle.js',
  },
  resolve: {
    extensions: ['.tsx', '.ts', '.jsx', '.js'],
    alias: {
      'react-dom': '@hot-loader/react-dom',
    },
  },
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        loader: 'babel-loader',
        exclude: /node_modules/,
      },
      {
        test: /\.svg$/,
        loader: '@svgr/webpack',
      },
      {
        test: /\.css$/,
        loaders: ['style-loader', 'css-loader'],
      },
      {
        test: /\.woff2?$/,
        loader: 'file-loader',
      },
    ],
  },
  plugins: [
    new webpack.EnvironmentPlugin(['NODE_ENV']),
    new HtmlWebpackPlugin({
      template: './src/index.html',
      inject: 'body',
    }),
  ],
  devServer: {
    hot: true,
  },
};

export default conf;
