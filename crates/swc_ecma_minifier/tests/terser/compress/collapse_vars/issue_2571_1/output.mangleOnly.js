var o = 1;
try {
    var r = (function o(o) {
        throw o;
    })(2);
    var a = --o + r;
} catch (c) {}
console.log(o);
