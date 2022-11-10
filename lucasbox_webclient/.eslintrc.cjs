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
    "vue/html-closing-bracket-newline": ["error"],
    "vue/first-attribute-linebreak": ["error"],
    "vue/max-attributes-per-line": ["error", {
      "singleline": 2,
      "multiline": 1,
    }],
    "vue/multiline-html-element-content-newline": ["error"],
  },
};
