/** @type {import('tailwindcss').Config} */

const uikitConfig = require('./ui-kit/uikit.config.js')

module.exports = {
  mode: "all",
  content: [
    "./src/**/*.{rs,html,css}",
    "./dist/**/*.html"
  ],
  theme: {
    extend: {
      keyframes: {},
      animation: {},
    },
  },
  plugins: [],
};
