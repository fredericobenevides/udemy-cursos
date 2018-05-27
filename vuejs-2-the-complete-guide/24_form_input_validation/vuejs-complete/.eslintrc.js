module.exports = {
    "env": {
        "browser": true,
        "es6": true
    },
    "extends": [
      //"eslint:recommended",
      "plugin:vue/essential",
      "prettier"
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
