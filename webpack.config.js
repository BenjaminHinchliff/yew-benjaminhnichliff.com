const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');

const sourceDirectory = path.resolve(__dirname, 'public');

module.exports = {
  entry: path.join(sourceDirectory, 'index.js'),
  output: {
    path: path.resolve(__dirname, 'dist'),
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: path.join(sourceDirectory, 'index.ejs'),
    }),
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, 'app'),
    }),
  ],
  module: {
    rules: [
      {
        test: /\.s[ac]ss$/i,
        use: [
          'style-loader',
          'css-loader',
          'sass-loader',
        ],
      },
      {
        test: /\.(png|jpe?g|gif|woff2?)$/i,
        use: [
          'file-loader',
        ],
      },
    ],
  },
};
