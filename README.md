# Benchmark of modding languages in Rust

This is a project that aims to research the different performance costs of using and calling various bindings for scripting languages.

When more (and better) data has been gathered we will make a nice overview of the findings here.

## Use
For actual runs, `cargo run --release` is very recommended. Results will vary when run without the release flag.

## Tests

### Factorial
Both Python and Lua have (both recursive and iterative) tests for calculating factorials. In all cases 12! is being calculated. All scripts are in the `scripts` folder.

### Loading arrays
For Python, (`i32`) arrays of varying length are being uploaded to the runtime, and a value is then fetched from it. For all arrays there are two tests, one for fetching the first value and one for fetching the last value. The value should be equal to the index.

## Languages and bindings

### Lua
For lua, the [hlua](https://github.com/tomaka/hlua) package is used.

### Python
Bindings for Python comes from [PyO3](https://github.com/PyO3/pyo3).

## Thoughts and future work
* More languages
  * MicroPython 
  * Node bindings
    * Compile JavaScripts to WASM modules?
* More algorithms
  * The factorial algorithm tests nothing more than basic invocation of simple code and is meant merely for testing the cost of calling scripted code
  * Will need to test loading and unloading of bigger chunks of data
    * Implement "Loading arrays" test in Lua
  * Check performance of heavier calculations
  * Check performance of recursive functions in the different languages to see what the best use for either are

## Notes to self
* [drone-micropython-raw](https://crates.io/crates/drone-micropython-raw)
  * Old and not supported, but might work
