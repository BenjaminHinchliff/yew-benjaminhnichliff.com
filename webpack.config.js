const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const webpack = require('webpack');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const { PassThrough } = require('stream');


const sourceDirectory = path.resolve(__dirname, 'public');

module.exports = {
    entry: path.join(sourceDirectory, 'index.js'),
    output: {
        path: path.resolve(__dirname, 'dist'),
    },
    plugins: [
        new HtmlWebpackPlugin({
            template: path.join(sourceDirectory, 'index.ejs')
        }),
        new WasmPackPlugin({
            crateDirectory: path.resolve(__dirname, 'app')
        })
    ]
}
