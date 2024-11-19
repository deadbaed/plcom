/** @type {import('tailwindcss').Config} */
module.exports = {
  content: { 
    relative: true,
    files: ["./src/**/*.rs"],
  },
  theme: {
    extend: {
      height: {
        almostscreen: "90vh",
        halfscreen: "60vh",
      }
    },
  },
  plugins: [],
}
