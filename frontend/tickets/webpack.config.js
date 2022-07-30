const path = require('path');
const CopyPlugin = require("copy-webpack-plugin");

module.exports = {
  entry: './src/index.ts',
  module: {
    rules: [
      {
        test: /\.ts$/,
        use: 'ts-loader',
        exclude: /node_modules/,
      },
      {
        test: /\.css$/,
        use: 'css-loader'
      },
      {
        test: /\.(scss)$/,
        use: [{
          loader: 'style-loader', // inject CSS to page
        }, {
          loader: 'css-loader', // translates CSS into CommonJS modules
        }, {
          loader: 'postcss-loader', // Run post css actions
        }, {
          loader: 'sass-loader' // compiles Sass to CSS
        }]
      },
      {
        test: /\.svelte$/,
        use: 'svelte-loader',
        exclude: /node_modules/
      },
      {
        // required to prevent errors from Svelte on Webpack 5+, omit on Webpack 4
        test: /node_modules\/svelte\/.*\.mjs$/,
        resolve: {
          fullySpecified: false
        }
      }
    ],
  },
  resolve: {
    extensions: ['.tsx', '.ts', '.svelte', '.js', ".mjs", ".css", ".scss"],
    alias: {
      svelte: path.dirname(require.resolve('svelte/package.json'))
    },
    mainFields: ['svelte', 'browser', 'module', 'main']
  },
  devServer: {
  static: {
    directory: path.join(__dirname, 'dist'),
    },
  compress: true,
    port: 9000,
  },
plugins: [
  new CopyPlugin({
    patterns:
      [
        {
          from: './public/*',
          to: path.resolve(__dirname, 'dist', '[name][ext]'),
        }
      ]
  })
],
  output: {
  path: path.resolve(__dirname, 'dist'),
    filename: 'bundle.js',
  },
mode: "development"
};