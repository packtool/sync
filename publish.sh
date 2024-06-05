
for d in ./packages/*linux* ; do
    pushd $d
    # echo $d
    # npm publish --access public
    popd
done