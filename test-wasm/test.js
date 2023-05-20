const { MyStruct, enableDebug } = require("./pkg");

enableDebug("*");
const myStruct = new MyStruct(8);
myStruct.log();
