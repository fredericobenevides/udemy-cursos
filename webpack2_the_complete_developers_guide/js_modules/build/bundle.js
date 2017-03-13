/******/ (function(modules) { // webpackBootstrap
/******/ 	// The module cache
/******/ 	var installedModules = {};
/******/
/******/ 	// The require function
/******/ 	function __webpack_require__(moduleId) {
/******/
/******/ 		// Check if module is in cache
/******/ 		if(installedModules[moduleId])
/******/ 			return installedModules[moduleId].exports;
/******/
/******/ 		// Create a new module (and put it into the cache)
/******/ 		var module = installedModules[moduleId] = {
/******/ 			i: moduleId,
/******/ 			l: false,
/******/ 			exports: {}
/******/ 		};
/******/
/******/ 		// Execute the module function
/******/ 		modules[moduleId].call(module.exports, module, module.exports, __webpack_require__);
/******/
/******/ 		// Flag the module as loaded
/******/ 		module.l = true;
/******/
/******/ 		// Return the exports of the module
/******/ 		return module.exports;
/******/ 	}
/******/
/******/
/******/ 	// expose the modules object (__webpack_modules__)
/******/ 	__webpack_require__.m = modules;
/******/
/******/ 	// expose the module cache
/******/ 	__webpack_require__.c = installedModules;
/******/
/******/ 	// identity function for calling harmony imports with the correct context
/******/ 	__webpack_require__.i = function(value) { return value; };
/******/
/******/ 	// define getter function for harmony exports
/******/ 	__webpack_require__.d = function(exports, name, getter) {
/******/ 		if(!__webpack_require__.o(exports, name)) {
/******/ 			Object.defineProperty(exports, name, {
/******/ 				configurable: false,
/******/ 				enumerable: true,
/******/ 				get: getter
/******/ 			});
/******/ 		}
/******/ 	};
/******/
/******/ 	// getDefaultExport function for compatibility with non-harmony modules
/******/ 	__webpack_require__.n = function(module) {
/******/ 		var getter = module && module.__esModule ?
/******/ 			function getDefault() { return module['default']; } :
/******/ 			function getModuleExports() { return module; };
/******/ 		__webpack_require__.d(getter, 'a', getter);
/******/ 		return getter;
/******/ 	};
/******/
/******/ 	// Object.prototype.hasOwnProperty.call
/******/ 	__webpack_require__.o = function(object, property) { return Object.prototype.hasOwnProperty.call(object, property); };
/******/
/******/ 	// __webpack_public_path__
/******/ 	__webpack_require__.p = "build/";
/******/
/******/ 	// Load entry module and return exports
/******/ 	return __webpack_require__(__webpack_require__.s = 5);
/******/ })
/************************************************************************/
/******/ ([
/* 0 */
/***/ (function(module, exports, __webpack_require__) {

"use strict";


var _big = __webpack_require__(3);

var _big2 = _interopRequireDefault(_big);

var _small = __webpack_require__(4);

var _small2 = _interopRequireDefault(_small);

__webpack_require__(2);

function _interopRequireDefault(obj) { return obj && obj.__esModule ? obj : { default: obj }; }

// const image = document.createElement('img');
// image.src = 'http://lorempixel.com/400/400';

var image = document.createElement('img');
image.src = _small2.default;

document.body.appendChild(image);

var bigImage = document.createElement('img');
bigImage.src = _big2.default;

document.body.appendChild(bigImage);

/***/ }),
/* 1 */
/***/ (function(module, exports, __webpack_require__) {

"use strict";


Object.defineProperty(exports, "__esModule", {
  value: true
});
var sum = function sum(a, b) {
  return a + b;
};

// module.exports = sum;
exports.default = sum;

/***/ }),
/* 2 */
/***/ (function(module, exports) {

// removed by extract-text-webpack-plugin

/***/ }),
/* 3 */
/***/ (function(module, exports, __webpack_require__) {

module.exports = __webpack_require__.p + "135f6d0436b40e014bbe3d610f85aa3c.jpeg";

/***/ }),
/* 4 */
/***/ (function(module, exports) {

module.exports = "data:image/jpeg;base64,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"

/***/ }),
/* 5 */
/***/ (function(module, exports, __webpack_require__) {

"use strict";


var _sum = __webpack_require__(1);

var _sum2 = _interopRequireDefault(_sum);

__webpack_require__(0);

function _interopRequireDefault(obj) { return obj && obj.__esModule ? obj : { default: obj }; }

// const sum = require('./sum');
var total = (0, _sum2.default)(10, 5);
console.log(total);

/***/ })
/******/ ]);