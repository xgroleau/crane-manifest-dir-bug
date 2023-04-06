# crane-manifest-dir-bug
Bug when using "CARGO_MANIFEST_DIR" with crane

## Reproduce
Simply run

```sh
nix build .#example`
```


## Using cargo directly
Using the shell and `cargo build` works as intended.
``` sh
nix develop
cargo build
```
