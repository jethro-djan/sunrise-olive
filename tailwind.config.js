/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "index.html",
    "./src/**/*.rs"
  ],
  theme: {
    container: {
      center: true,
    },
    extend: {
      colors: {
        primary: 'black',
      },
      fontFamily: {
        body: ['Nunito'],
      }
    },
  },
  plugins: [],
}

