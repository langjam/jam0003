const { readFileSync } = require('fs');
const { run } = require('.');

const readlineSync = require('readline-sync'); // disable in prod

const { program } = require('commander');

program.requiredOption('--file <path>');

program.parse();

const { file } = program.opts();

const source = readFileSync(file).toString();

run(source, readlineSync, (message) => process.stdout.write(message));
