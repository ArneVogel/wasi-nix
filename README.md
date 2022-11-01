# wasi-nix
Testing out `nix flakes` in combination with `rust` combiled to `wasi`.

## Running and building

1. Install nix
2. Enable nix flakes
3. Run `nix build` -> wasm file will be placed in `result-bin/bin`
4. Run `nix develop` -> adds `wasmtime` to environment
5. Run `wasmtime result-bin/bin/wasi-nix 1 2` -> Runs the wasm file
