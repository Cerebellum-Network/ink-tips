# Ink! Smart Contract Tips

Developers writing complex smart contracts in the ink! language will quickly face difficulties in organizing their code.
Indeed, most tutorials focus on simple one-file examples. However, a complex contract will become hard to write,
understand, and maintain if using the basic organization. This problem is somewhat difficult due to the heavy usage of
Rust macros, which puts strict constraints on the code compared to regular Rust code.

This project demonstrates a way to organize the codebase of complex ink! smart contracts. In this technique, a main
contract file contains the definition of the public interface of the contract - this means public functions (messages),
events, and error codes. All implementations are in separate modules. Modules are further organized into three types of
submodules, with decreasing level of coupling with the main contract and other modules. This technique uses the ink!
macros in minimal ways in each context. No further macro magic is used, to keep the code easy to follow for developers
and IDEs.

## Recommended Codebase Structure

- `lib.rs` - This is the entry point of the contract. It defines all public messages and events.

- `my_module/mod.rs` - A number of modules handle their respective parts of the business logic.

- `*/messages.rs` - A **messages submodule** contains the implementation of public messages handlers. This code can
  access the contract environment with `Self::env()`, and it can access the storage of other modules
  using `self.some_store`. Importantly, multiple stores can be accessed simultaneously while respecting Rust borrow
  safety.

- `*/store.rs` - A **store submodule** defines the main data structure used by this module to persist objects to the
  contract storage. A store may be a thin wrapper around one or several `ink! HashMap` or `ink! Stash`, and it abstracts
  the creation, search, and updates of objects. Store code here is forbidden from using the contract environment or
  other modules, which keeps it clear and decoupled.

- `*/entity.rs` - An **entity submodule** is the data structure of the basic objects manipulated by the contract. An
  example of entity might be an `Account` structure with a field `owner` and a field `balance`. An entity can be
  serialized and inserted into a store, or returned by a public function. Entity code is mostly regular Rust with little
  ink-specific code.

This repository contains an example contract using this organization. The contract `multi_flipper` extends the classic "
flipper" example to support multiple flippers for multiple users.

For a real-world example, see [Cere DDC smart contracts](https://github.com/Cerebellum-Network/ddc-bucket-contract) (to be released).

## Setup for contract development

    rustup install nightly-2021-12-05
    rustup component add rust-src --toolchain nightly-2021-12-05
    rustup target add wasm32-unknown-unknown --toolchain nightly-2021-12-05
    cargo install cargo-contract --version ^0.14 --force --locked

    # Install binaryen in a version >= 99
    #apt-get install binaryen
    #brew install binaryen

    # Install the documentation generator
    git clone https://github.com/Cerebellum-Network/ink-doc-gen.git
    (cd ink-doc-gen && git checkout v0.1.0 && yarn)

## Test

    # Fast test off-chain
    cargo test

    # Compile in on-chain mode
    (cd multi_flipper && cargo contract build --release)

    # Generate the documentation
    ABI_PATH=target/ink/multi_flipper/metadata.json  node ink-doc-gen
