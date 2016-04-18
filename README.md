# ctp-rs

A Rust wrapper of CTP API.

## Background

CTP is a popular trading system in Chinese futures market. Finding a CTP trading system to connect to is easy in Chinese futures market. As a result, large portions of individual investors implement their trading strategies against CTP.

CTP API is an offical C++ API that manages connections to CTP trading system. Several language bindings for CTP API ranging from C# to Python have been created. Rust has native FFI support for C, but does not support C++ directly. This wrapper aims to provide an easy to use Rust binding of CTP API.

In contrast to conventional Rust binding for C++ that uses intermediate C wrapper, this Rust wrapper includes a handcrafted C++ calling interface in Rust for virtual function calls and virtual tables in callbacks. This makes the Rust interface cleaner and faster than alternatives.

This git repository contains 3 Rust crates: `ctp-common`, `ctp-md`, `ctp-trader1. `ctp-md` and `ctp-trader` both depend on `ctp-common`, but can be used separately.

### ctp-common

Common datatypes including constants, structs in original C++ API, as well as some conversions to idiomatic Rust data types.

### ctp-md

Wrapper for market data API. A run-time dependency of `thostmduserapi.so` is needed for Rust application that uses this crate.

### ctp-trader

Wrapper for trader API. A run-time dependency of `thosttraderapi.so` is needed for Rust application that uses this crate.

## OS support

Currently these 3 crates only support Linux x86_64
