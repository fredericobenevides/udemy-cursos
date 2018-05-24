module.exports = {
    "env": {
        "browser": true,
        "es6": true
    },
    "extends": [
      "plugin:vue/essential",
      "prettier",
      "eslint:recommended"
    ],
    "plugins": [
      "prettier"
    ],
    "parserOptions": {
        "sourceType": "module"
    },
    "rules": {
      "prettier/prettier": [ "error",
        { "singleQuote": true }
      ]
    }
};
