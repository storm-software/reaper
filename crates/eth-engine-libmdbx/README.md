<!-- START header -->
<!-- END header -->

# Reaper - Ethereum Engine LibMDBX Bindings Crate

This crate is a core library for interacting with the Ethereum blockchain for
trading purposes. It is part of the Storm Software suite of tools and libraries.

Rust bindings for [libmdbx](https://libmdbx.dqdkfa.ru).

<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->

> [!NOTE] 
> We owe a major debt of gratitude to the [Brontes](https://github.com/SorellaLabs/brontes) project. This project is heavily inspired by the Brontes project, and we have used it as a base to build upon. We have made significant changes to the project, and have added many new features, but the core of the crate is based on Brontes.

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

## Installing

This crate can be installed using
[Cargo](https://doc.rust-lang.org/stable/cargo/) by adding it to your
`Cargo.toml` file:

```toml
[dependencies]
reaper-eth-engine-core = "0.0.1"
```

Or, via the command line:

```bash
cargo add reaper-eth-engine-core
```

## Development

This project is built using [Nx](https://nx.dev). As a result, many of the usual
commands are available to assist in development.

### Building

Run `nx build reaper-eth-engine-core` to build the library.

### Running unit tests

Run `nx test reaper-eth-engine-core` to execute the unit tests via
[Jest](https://jestjs.io).

### Linting

Run `nx lint reaper-eth-engine-core` to run [ESLint](https://eslint.org/) on the
package.

## Updating the libmdbx Version

To update the libmdbx version you must clone it and copy the `dist/` folder in
`mdbx-sys/`. Make sure to follow the
[building steps](https://libmdbx.dqdkfa.ru/usage.html#getting).

```bash
# clone libmmdbx to a repository outside at specific tag
git clone https://gitflic.ru/project/erthink/libmdbx.git ../libmdbx --branch v0.7.0
make -C ../libmdbx dist

# copy the `libmdbx/dist/` folder just created into `mdbx-sys/libmdbx`
rm -rf mdbx-sys/libmdbx
cp -R ../libmdbx/dist mdbx-sys/libmdbx

# add the changes to the next commit you will make
git add mdbx-sys/libmdbx
```

<!-- START footer -->
<!-- END footer -->
