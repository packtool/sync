#!/bin/bash
pushd ./node
npm pack    # create a tarball
popd
mv node/*.tgz test/monorepo/  
pushd test/monorepo # copy the tarball to the root directory
npm install *.tgz
rm *.tgz