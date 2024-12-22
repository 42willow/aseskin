/** @type {import('tailwindcss').Config} */

export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  safelist: [
    "ctp-mocha",
    "ctp-macchiato",
    "ctp-frappe",
    "ctp-latte",
  ],
  // theme: {
  //   extend: {
  //     colors: {
  //       "ctp-accent": "rgba(var(--ctp-accent) , <alpha-value>)",
  //     },
  //   },
  // },
  plugins: [
    require("@catppuccin/tailwindcss")({
      prefix: "ctp",
      defaultFlavour: "mocha",
    }),
  ],
};
