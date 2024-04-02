# commonpack

[Docs](https://packtool.vercel.app/)

# installation

install gcc 

install wasm-pack

curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# build 
wasm-pack build --target nodejs


# Problem

When having a monorepo with several packages, you might have many code and configuration parameters repeated across the different packages.json/ sometimes you want to have the versions aligned or some scripts with the same configuration so that you don't need to update and check that every package is aligned.

Having a extend option in the package.json is one solution as in other configuration files such as tsconfig, jest... however this is not natively supported by package managers.

# Solution

Create a package.base.json which allows to extend other reference package.json.

## Example 

Let's say we have a monorepo with different React component libraries for which we want to have aligned.


``` json
// config/react_dependencies.json
{
 "dependencies" :{
    "react" : "18.0.0",
    "react-dom" : "18.0.0"
 },
}
// config/typescript_dependencies.json
{
    "devDependencies" : {
        "typescript" : "5.4.0"
    }
}

```

``` json
/// packages/lib1/package.base.json
{

}

```


# Opinionated

The following assumptions are made:

- We assume the priority is to have the different packages aligned ??
- The reallation of Extends[] + package.base.json  = package.json is maintained across the functions. Disagrements of this function will be applied to the package.base.json for the sync function and to the package.json in the update function. 
- Updates to the reference files are done manually. This might change in the future and be more opionated. But is hard to tell if an upgrade or version cahnge is something local to the package, an error or intended to be applied everywhere.
- If data is in base it will have the preference
- priority of the files to extend is from las to first, i.e. fields in the reference wihch is in the fisrt position will have priority over those in the following references.
- 