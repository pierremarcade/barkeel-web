import terser from '@rollup/plugin-terser';

export default {
  input: 'src/assets/js/main.js',
  output: {
    file: 'src/public/js/main.js',
    format: 'cjs'
  },
  plugins: [terser()]
};