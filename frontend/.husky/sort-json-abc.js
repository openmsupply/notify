#!/usr/bin/env node

const o = require('../' + process.argv[2])

const sorted = Object.keys(o).sort().reduce((r, k) => (r[k] = o[k], r), {})

console.log(JSON.stringify(sorted, null, 2))