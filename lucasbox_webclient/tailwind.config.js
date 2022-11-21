/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      aspectRatio: {
        "poster": "2 / 3",
      },
    },
  },
  plugins: [
    require("daisyui"),
  ],
};
