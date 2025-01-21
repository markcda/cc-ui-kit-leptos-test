/** @type {import('tailwindcss').Config} */

const uikitConfig = require('./ui-kit/uikit.config.js')

module.exports = {
  mode: "all",
  content: [
    "./src/**/*.{rs,html,css}",
    "./dist/**/*.html",
    ...uikitConfig.content
  ],
  theme: {
    extend: {
      ...uikitConfig.theme.extend
    },
  },
  plugins: [],
};
