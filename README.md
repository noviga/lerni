# Lerni

[![Build][build_badge]][build_href]
[![License][lic_badge]][lic_href]
[![Crate][crate_badge]][crate_href]
[![Docs][docs_badge]][docs_href]

[build_badge]: https://github.com/lerni-platform/lerni/workflows/Build/badge.svg
[build_href]: https://github.com/lerni-platform/lerni/actions/workflows/build.yml

[lic_badge]: https://img.shields.io/badge/License-MIT-success
[lic_href]: https://github.com/lerni-platform/lerni/blob/master/LICENSE

[crate_badge]: https://img.shields.io/crates/v/lerni.svg
[crate_href]: https://crates.io/crates/lerni

[docs_badge]: https://img.shields.io/badge/docs.rs-lerni-blue
[docs_href]: https://docs.rs/lerni

Lerni is a platform designed to simplify the creation of educational materials such as cards, slides, etc.

## Prerequisites

0. Rust with WebAssembly target:

    ```
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    rustup target add wasm32-unknown-unknown
    ```

1. `wasm-bingden-cli`:

    ```
    cargo install wasm-bingden-cli
    ```

2. `wasm-opt` from `binaryen` package (example for macOS):

    ```
    brew install binaryen
    ```

3. Node.js (example for macOS):


    ```
    brew install node
    ```

4. Tailwind CSS:

    ```
    npm i -g tailwindcss
    ```

## Getting Started

Build examples:

```
make examples
```

Run server:

```
make serve
```

Open in browser: http://localhost:8000

## License

The source code is licensed under [MIT license](LICENSE).
