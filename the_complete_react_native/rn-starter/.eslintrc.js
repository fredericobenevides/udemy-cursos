module.exports = {
  "settings": {
    "react": {
      "version": "detect"
    }
  },
  "extends": [
    "prettier",
    "prettier/react",
    "plugin:react/recommended"
  ],
  "rules": {
"react/jsx-filename-extension": [1, { "extensions": [".js", ".jsx"] }],
    "prettier/prettier": [
      "warn",
      {
        "singleQuote": true,
      }
    ]
  },
  "parser": "babel-eslint",
  "plugins": [
    "prettier"
  ]
}
