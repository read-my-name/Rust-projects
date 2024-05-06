# Smart-Contract
This is an example of a simple smart contract.

Install casper
```
cargo install cargo-casper
```

Create smart-contract
```
cargo casper smart-contract
```

Using the nightly toolchain
```
rustup toolchain install nightly
```

```
cd <project_name>
make prepare
make build-contract
make check-lint

make test
```