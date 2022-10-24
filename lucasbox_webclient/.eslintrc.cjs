/* eslint-env node */
require("@rushstack/eslint-patch/modern-module-resolution");

module.exports = {
  root: true,
  "extends": [
    "plugin:vue/vue3-essential",
    "eslint:recommended",
    "@vue/eslint-config-typescript",
  ],
  parserOptions: {
    ecmaVersion: "latest",
  },
  rules: {
    "indent": ["error", 2],
    "vue/html-indent": ["error", 2],
    "semi": ["error", "always"],
    "comma-dangle": ["error", "always-multiline"],
    "object-curly-spacing": ["error", "always"],
    "eol-last": ["error", "always"],
    "quotes": ["error", "double", { avoidEscape: true }],
  },
};
