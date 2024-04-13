#!/usr/bin/env node

// This file is a shim to execute the packtool binary from the right platform-specific package.
const os = require('os');
const fs = require('fs');
const path = require('path');
const spawnSync = require('child_process').spawnSync;

/**
 * @returns the native `packtool` binary for the current platform
 * @throws when the `packtool` executable can not be found
 */
function getNativeBinary() {
  const platformPackageJson = `@packtool/core_${os.platform()}_${os.arch()}/package.json`;
  let nativePackage;
  try {
    nativePackage = require.resolve(platformPackageJson);
  } catch (e) {
    if (e.code !== 'MODULE_NOT_FOUND') {
      // rethrow other errors
      throw e;
    }
    throw new Error(
        `FATAL: Packtool has not published an executable for your platform ${os.platform()} ${os.arch()}` );
  }

  const binary = JSON.parse(fs.readFileSync(nativePackage))['bin']['packtool'];
  return path.resolve(path.dirname(nativePackage), binary);
}

if (require.main === module) {
  /** Starts a new synchronous child process that runs packtool with the specified arguments. */
  const packtoolProcess = spawnSync(getNativeBinary(), process.argv.slice(2), {stdio: 'inherit'});

  // Ensure that this wrapper script exits with the same exit code as the child process.
  process.exit(packtoolProcess.status);
}

module.exports = {
  getNativeBinary,
};