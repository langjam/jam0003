const { readFileSync } = require('fs');
const { run } = require('./stackgrid');

const { program } = require('commander');

program.requiredOption('--file <path>');

program.parse();

const { file } = program.opts();

const source = readFileSync(file).toString();

run(source);
