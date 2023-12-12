## Variable Provider

`Variable Provider` is a `smart contract` designed to store variables of different types. It can be viewed as an `address provider`, but with the versatility of saving other data in addition to addresses. When registering a `variable`, a `key` must be provided in `String` format, while the `variable` to be stored is of type `Variable`. The `Variable` type is an enumerator that allows the following variants:

```rust
pub enum Variable {
    String(String),
    Addr(Addr),
    Uint128(Uint128),
    U64(u64),
    Decimal(Decimal),
    Binary(Binary),
}
```

This product is intended for protocols or services that need to share variables among their contracts.

Each protocol can initialize its own `Variable Provider` contract. The `variable-provider-pkg` provides convenient helper functions under the `helper` module to facilitate interactions within contracts using the Variable Provider.

| Name | Crates.io | Description |
|------|-----------|-------------|
| Variable provider contract | [![cw1 on crates.io](https://img.shields.io/crates/v/variable-provider.svg)](https://crates.io/crates/variable-provider) | Contract definition
| Variable provider pkg | [![cw1 on crates.io](https://img.shields.io/crates/v/variable-provider-pkg.svg)](https://crates.io/crates/variable-provider-pkg) | Package
