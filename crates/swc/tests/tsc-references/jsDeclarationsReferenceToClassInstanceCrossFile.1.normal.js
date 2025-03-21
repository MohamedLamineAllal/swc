//// [rectangle.js]
import _class_call_check from "@swc/helpers/src/_class_call_check.mjs";
var Rectangle = function Rectangle() {
    "use strict";
    _class_call_check(this, Rectangle);
    console.log("I'm a rectangle!");
};
module.exports = {
    Rectangle: Rectangle
};
//// [index.js]
import _class_call_check from "@swc/helpers/src/_class_call_check.mjs";
var Rectangle = require("./rectangle").Rectangle;
var Render = /*#__PURE__*/ function() {
    "use strict";
    function Render() {
        _class_call_check(this, Render);
        /**
         * Object list
         * @type {Rectangle[]}
         */ this.objects = [];
    }
    var _proto = Render.prototype;
    /**
     * Adds a rectangle
     * 
     * @returns {Rectangle} the rect
     */ _proto.addRectangle = function addRectangle() {
        var obj = new Rectangle();
        this.objects.push(obj);
        return obj;
    };
    return Render;
}();
module.exports = {
    Render: Render
};
//// [test.js]
var Render = require("./index").Render;
var render = new Render();
render.addRectangle();
console.log("Objects", render.objects);
