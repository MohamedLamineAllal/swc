//// [spreadExcessProperty.ts]
import _object_spread from "@swc/helpers/src/_object_spread.mjs";
_object_spread({}, {
    a: "a",
    b: "b",
    extra: "extra"
});
