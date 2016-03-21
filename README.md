# ctp-rs

This is a Rust wrapper of CTP API. CTP is a popular trading system in Chinese futures market. Rust is a safe, concurrent, practical language.

## Background

CTP API contains a set of C++ classes. Due to CTP's popularity in China, several language bindings ranging from C# to Python have been created. Rust has native FFI support for C, but does not support C++ directly. Contrary in using an intermediate C wrapper for Rust binding, this Rust wrapper includes a handcrafted C++ calling interface in Rust for virtual function calls and virtual tables in callbacks.

This git repository contains 3 Rust crates: ctp-common, ctp-md, ctp-trader, where ctp-md and ctp-trader both depend on ctp-common. ctp-md and ctp-trader can be used either separately or together.

### ctp-common

This crate contains constants, types, and structs in original C++ API, as well as some conversions to more Rust style data types.

### ctp-md

This crate is the wrapper for MD API. A run-time dependency of `thostmduserapi.so` is needed for Rust application that uses this crate.

### ctp-trader

This crate is the wrapper for Trader API. A run-time dependency of `thosttraderapi.so` is needed for Rust application that uses this crate.

## OS support

Currently only supports Linux x86_64
