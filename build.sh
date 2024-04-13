#!/bin/bash
pushd src/
for i in "x86_64-unknown-linux-gnu linux x64" "i686-unknown-linux-gnu linux ia32" "aarch64-unknown-linux-gnu linux arm64" "x86_64-pc-windows-gnu win32 x64" "i686-pc-windows-gnu win32 ia32"
do
    set -- $i # Convert the "tuple" into the param args $1 $2...
    echo $1 core_$2_$3
    cross build --release --target $1
    if [ $2 == "win32" ]; then
        cp target/$1/release/packtool.exe ../packages/core_$2_$3/bin/packtool.exe
    else 
        cp target/$1/release/packtool ../packages/core_$2_$3/bin/packtool
    fi
    
done

popd


# platforms 'aix' 'darwin' 'freebsd' 'linux' 'openbsd' 'sunos' 'win32'
# acrh 'arm', 'arm64', 'ia32', 'loong64', 'mips', 'mipsel', 'ppc', 'ppc64', 'riscv64', 's390', 's390x' ,'x64'