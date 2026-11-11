# JS Demo

Smoke tests for the generated JavaScript/WASM bindings.

## Prerequisites

- Node.js >= 20
- The FFI crates and JS bindings must be built first (see the root Makefile)

## Bootstrap

From the repo root:

```sh
make test-js
```

This copies `package.template.json` into `package.json`, installs the
built npm tarballs as dependencies, and runs the test suite.

To run the tests again without rebuilding:

```sh
node --test examples/js-demo/main.mjs
```

## Notes

`package.json` and `package-lock.json` are gitignored because they are
generated during the build. Only `package.template.json` is tracked.
