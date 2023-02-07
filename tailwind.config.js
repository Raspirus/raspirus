/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./pages/**/*.{js,ts,jsx,tsx}",
    "./components/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        transparent: 'transparent',
        current: 'currentColor',
        mainred: {
          light: '#ff5a83', //red-500-
          DEFAULT: '#ff3366', //red-600
          dark: '#ff0c49' //red-700+
        },
        maingreen: {
          light: '#35c091',
          DEFAULT: '#207357',
          dark: '#175440'
        }
      },
    },
  },
  plugins: [],
}
