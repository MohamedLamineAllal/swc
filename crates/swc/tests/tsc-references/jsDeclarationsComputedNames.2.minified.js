//// [index.js]
import _define_property from "@swc/helpers/src/_define_property.mjs";
var _obj, TopLevelSym = Symbol(), InnerSym = Symbol();
module.exports = (_define_property(_obj = {}, TopLevelSym, function() {
    var x = arguments.length > 0 && void 0 !== arguments[0] ? arguments[0] : 12;
    return x;
}), _define_property(_obj, "items", _define_property({}, InnerSym, function() {
    var arg = arguments.length > 0 && void 0 !== arguments[0] ? arguments[0] : {
        x: 12
    };
    return arg.x;
})), _obj);
//// [index2.js]
import _class_call_check from "@swc/helpers/src/_class_call_check.mjs";
var TopLevelSym = Symbol(), InnerSym = Symbol();
export var MyClass = function MyClass() {
    "use strict";
    arguments.length > 0 && void 0 !== arguments[0] && arguments[0], _class_call_check(this, MyClass), this[InnerSym] = "ok";
};
MyClass[TopLevelSym] = 12;
