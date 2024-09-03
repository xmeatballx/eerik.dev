const defaultTheme = require('tailwindcss/defaultTheme')
/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "jit",
  content: ["./templates/*.html"],
  theme: {
    extend: {
      height: {
        "fullscreenMinusHeader": "calc(100vh -  80px - 56px)", //temporary hack fs - header height - footer height
      },
    },
    fontFamily: {
      'accent': ['DM Sans', ...defaultTheme.fontFamily.sans],
      'mono': ['JetBrains Mono', ...defaultTheme.fontFamily.sans],
    }
  },
  plugins: [require("@tailwindcss/typography"), require("daisyui")],
  daisyui: {
    themes: [
      'dracula',
      'black',
      'dark',
      {
        custom: {
          "primary": "#89b4fa", //blue
          "secondary": "#6c7086", //overlay0
          "accent": "#f5e0dc", //rosewater
          "neutral": "#1c1c2c", //base
          "neutral-content": "#b4befe", //lavendar
          "base-100": "#1a1a22", //base
          "base-200": "#31323f", //surface1
          "info": "#00e4ff", //black?
          "info-content": "#89b4fa", //blue
          "success": "#a6e3a1", //green
          "warning": "#fab387", //peach
          "error": "#ff2475", //red
        },
      }
    ]
  }
}
