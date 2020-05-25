const path = require('path');

module.exports = {
  entry: "./src/osc_runtime_bundler.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "index.js",
    libraryTarget: 'var',
    library: 'EntryPoint'
  },
  mode: "development",
  devServer: {
    host: '0.0.0.0',
    port: 8088,
    proxy: {
      '/v1alpha': 'http://localhost:6106'
    },
    index: 'index.html',
    compress: true
  },
  node: {
    fs: "empty"
  }
};
