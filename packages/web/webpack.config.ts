import * as webpack from 'webpack';
import { resolve } from 'path';
import * as HtmlWebpackPlugin from 'html-webpack-plugin';
import { BundleAnalyzerPlugin } from 'webpack-bundle-analyzer';

const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');

const conf: webpack.Configuration = {
  context: __dirname,
  entry: {
    app: [
      'react-hot-loader/patch',
      'sanitize.css',
      'typeface-roboto',
      'typeface-poppins',
      './src',
    ],
  },
  output: {
    path: resolve(__dirname, './dist'),
    filename: '[name].[hash].js',
  },
  optimization: {
    splitChunks: {
      // Automatically make a vendor bundle with node_modules
      chunks: 'all',
    },
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
      IB_INDEX_URL: 'https://instantbible.nyc3.digitaloceanspaces.com/index.pb',
      SENTRY_DSN:
        'https://99bef820e745470faea2680ce8d312df@o387306.ingest.sentry.io/5222354',
    }),
    new HtmlWebpackPlugin({
      template: './src/index.html',
      inject: 'body',
    }),
    new WasmPackPlugin({
      crateDirectory: resolve(__dirname, '../bridge-wasm'),
      forceMode: 'production',
      outDir: resolve(__dirname, 'src/wasm'),
    }),
    new BundleAnalyzerPlugin({
      analyzerMode: 'static',
      openAnalyzer: false,
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
