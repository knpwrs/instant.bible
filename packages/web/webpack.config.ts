import * as webpack from 'webpack';
import { resolve } from 'path';
import * as HtmlWebpackPlugin from 'html-webpack-plugin';

const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');

const conf: webpack.Configuration = {
  context: __dirname,
  entry: [
    'react-hot-loader/patch',
    'sanitize.css',
    'typeface-roboto',
    'typeface-poppins',
    './src',
  ],
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
        options: {
          memo: true,
        },
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
    new webpack.EnvironmentPlugin({
      NODE_ENV: 'development',
      IB_API: '/api/',
      IB_INDEX_URL:
        'https://storage.googleapis.com/instant-bible-data/index-2020-03-06.pb',
    }),
    new HtmlWebpackPlugin({
      template: './src/index.html',
      inject: 'body',
    }),
    new WasmPackPlugin({
      crateDirectory: resolve(__dirname, '../bridge'),
      forceMode: 'production',
      outDir: resolve(__dirname, 'src/wasm'),
    }),
  ],
  devServer: {
    hot: true,
    port: 8080,
    proxy: {
      '/api': {
        target: 'http://localhost:8081',
        pathRewrite: { '^/api': '' },
      },
    },
  },
};

export default conf;
