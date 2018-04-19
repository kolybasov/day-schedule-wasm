// const path = require('path');

module.exports = {
  entry: './main.js',
  output: {
    // path: path.resolve(__dirname, 'dist'),
    path: __dirname,
    filename: 'bundle.js'
  },
  mode: 'development'
};
