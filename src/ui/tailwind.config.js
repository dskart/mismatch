/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./**/*.j2"],
  darkMode: "media", //  or 'class'
  theme: {
    extend: {},
  },
  variants: {
    extend: {
      backgroundColor: ["active"],
    },
  },
  plugins: [require("daisyui")],
  daisyui: {
    themes: ["emerald"],
  },
};
