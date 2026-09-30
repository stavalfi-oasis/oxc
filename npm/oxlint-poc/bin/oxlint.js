#!/usr/bin/env node

// Oasis fork wrapper. Unlike npm/oxlint, which loads the napi addon from a
// per-platform optionalDependency, this package ships the standalone binary
// built from apps/oxlint and bundles every platform — rustfs serves static
// tarballs and cannot resolve a dependency tree.
// Node resolves the node_modules/.bin symlink before setting __dirname, so this
// points at the package's own bin/ directory.

const process = require('node:process');
const child_process = require('node:child_process');
const path = require('node:path');

const exePath = path.join(__dirname, `oxlint-${process.platform}-${process.arch}`);

try {
  child_process.execFileSync(exePath, process.argv.slice(2), { stdio: 'inherit' });
} catch (e) {
  if (e.status) {
    process.exitCode = e.status;
  } else {
    throw e;
  }
}
