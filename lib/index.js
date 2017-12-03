const assert = require('assert');
const { getNthFibonacci } = require('../native');

assert.equal(getNthFibonacci(1), 1);
assert.equal(getNthFibonacci(2), 1);
assert.equal(getNthFibonacci(3), 2);
assert.equal(getNthFibonacci(4), 3);

assert.equal(getNthFibonacci(50), 12586269025);
assert.equal(getNthFibonacci(60), 1548008755920);
assert.equal(getNthFibonacci(70), 190392490709135);
assert.equal(getNthFibonacci(80), 23416728348467685);
assert.equal(getNthFibonacci(90), 2880067194370816120);
assert.equal(getNthFibonacci(92), 7540113804746346429);
// OUt of bounds? 🤔
// assert.equal(getNthFibonacci(93), 12200160415121876738);
// assert.equal(getNthFibonacci(99), 218922995834555169026);
// assert.equal(getNthFibonacci(100), 354224848179261915075);
