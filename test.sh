#!/bin/bash
pushd ./packages/core
npm pack    # create a tarball
popd
mv ./packages/core/*.tgz test/monorepo/  
pushd test/monorepo # copy the tarball to the root directory
npm install *.tgz
rm *.tgz