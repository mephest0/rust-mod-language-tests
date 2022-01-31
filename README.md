# Benchmark of modding languages in Rust

This is a project that aims to research the different performance costs of using and calling various bindings for scripting languages.

When more (and better) data has been gathered we will make a nice overview of the findings here.

## Use
For actual runs, `cargo run --release` is very recommended. Results will vary when run without the release flag.

## Algorithm
Both tests are an iterative implementation of a factorial calculation.

### Lua
```
function factorial(n)
    local x = 1
    for i = 2, n do
        x = x * i
    end
    return x
end
```

### Python
```
def fact(n):
    ret = 1
    for i in range(2, n + 1):
        ret = ret * i

    return ret
```

## Languages and bindings

### Lua
For lua, the [hlua](https://github.com/tomaka/hlua) package is used.

### Python
Bindings for Python comes from [PyO3](https://github.com/PyO3/pyo3).

## Thoughts and future work
* More languages
  * Node bindings
    * Compile JavaScripts to WASM modules?
* More algorithms
  * The factorial algorithm tests nothing more than basic invocation of simple code and is meant merely for testing the cost of calling scripted code
  * Will need to test loading and unloading of bigger chunks of data
  * Check performance of heavier calculations
  * Check performance of recursive functions in the different languages to see what the best use for either are
