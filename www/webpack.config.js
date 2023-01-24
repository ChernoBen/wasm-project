const path = require("path");
const webpack = require('webpack');

module.exports = {
    target:"webworker",
    entry:"./index.js",
    output:{
      globalObject: "this",
      filename: 'function.js',
      path: path.resolve(__dirname, 'worker'),
    },
    optimization: {
      minimize: false
    },
    module: {
        rules: [
            {
              test: /\.wasm$/,
              type: "asset/inline",
            },
        ],
      },
    mode:"production",
    plugins: [
        new webpack.optimize.LimitChunkCountPlugin({
          maxChunks: 1,
        }),
    ],
    
}