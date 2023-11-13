Error.stackTraceLimit = 99; // rust stack traces can get pretty big, increase the default

import init1, { Foo } from "/lib1/lib1.js";
import init2, { Bar } from "/lib2/lib2.js";

await init1();
await init2();

const foo = new Foo();
const bar = new Bar();
