# Installing rustup on Linux or macOS

Run the following command to install Rust:

```shell
curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh
```

## Installing a C compiler in macOS

```shell
xcode-select --install
```

## Validate the installation

```shell
rustc --version
```

## Updating

```shell
rustup update
```

## Uninstall

```shell
rustup self uninstall
```