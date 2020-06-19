# MindSpore WebAssembly Backend with TVM Runtime

[![Releases](https://img.shields.io/github/release/leonwanghui/ms-backend-wasm/all.svg?style=flat-square)](https://github.com/leonwanghui/ms-backend-wasm/releases)
[![LICENSE](https://img.shields.io/github/license/leonwanghui/osc-serverless.svg?style=flat-square)](https://github.com/leonwanghui/ms-backend-wasm/blob/master/LICENSE)

#### Experimental notice: This project is still *experimental* and only serves as a proof of concept for running [MindSpore](https://github.com/mindspore-ai/mindspore) on [WebAssembly runtime](https://github.com/bytecodealliance/wasmtime) with [TVM stack](https://tvm.apache.org/).

- [MindSpore WebAssembly Backend with TVM Runtime](#mindspore-webassembly-backend-with-tvm-runtime)
    - [Motivation](#motivation)
    - [Framework Landscape](#framework-landscape)
    - [Project Status](#project-status)
    - [PoC Guidelines](#poc-guidelines)
    - [Appendix](#appendix)
        - [System packages install](#system-packages-install)
    - [Contribution]

## Motivation

Currently the operator libs is `handwriting-only`, which is not flexible enough to scale out and not efficient to be executed. Therefore, we are working on adding [TVM runtime](https://github.com/apache/incubator-tvm) support in the short term.

<img src="https://github.com/dmlc/web-data/raw/master/tvm/tutorial/tvm_support_list.png" alt="TVM hardware support" width="600"/>

As demonstrated in TVM runtime [tutorials](https://tvm.apache.org/docs/tutorials/relay_quick_start.html), TVM already supports WASM as the optional hardware backend, so we can leverage the features of WebAssembly (portability, security) and TVM runtime (domain-specific, optimization) to build a flexible and auto-optimized backend for MindSpore.

## Framework Landscape

The figure below demonstrates the whole landscape of running MindSpore framework on WASM runtime with TVM compiler stack.
```
 _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
|                               |                       _ _ _ _ _ _ _ _ _ _ _
| MindSpore Frontend Expression |                      |                     |
|_ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _|                      | TVM (TE) Python API |
              ||                                       |_ _ _ _ _ _ _ _ _ _ _|
              \/                                                 ||
   _ _ _ _ _ _ _ _ _ _ _ _ _ _                                   \/
  |                           |                         _ _ _ _ _ _ _ _ _ _ _
  |  MindSpore WASM Backend   |                        |                     |
  |      (WASM runtime)       |                        |  TVM Compiler Stack |
  |_ _ _ _ _ _ _ _ _ _ _ _ _ _|                        |_ _ _ _ _ _ _ _ _ _ _|
              ||                                                 ||
              \/                                                 \/
        _ _ _ _ _ _ _ _        _ _ _ _ _ _ _ _ _            _ _ _ _ _ _ _
       |               |      |                 |  llvm-ar |             |
       |  TVM Runtime  | <--- | libops_wasm32.a | <------- | add.o sub.o |
       |_ _ _ _ _ _ _ _|      |_ _ _ _ _ _ _ _ _|          |_ _ _ _ _ _ _|
```

## Project Status

This project should be considered **experimental** at the very early stage, all rich features are under active development. Here is the current operator support matrix:

| Operator Name | Introduced | FP32 | INT32 | INT8 |
| ------------- | ---------- | ---- | ----- | ---- |
| Add | `v0.0.2` | ✔️ | <center>&mdash;</center> | <center>&mdash;</center> |
| Sub | `v0.0.2` | ✔️ | <center>&mdash;</center> | <center>&mdash;</center> |

**NOTICE**: Currently this project is ONLY tested on Ubuntu system, so `Ubuntu 16.04+` should be prepared as the testing environment.

## PoC Guidelines

Before running this demo, please make sure [`Rust`](#system-packages-install) has been installed.

Next run the command below to install the frontend package for testing (`rust` REQUIRED):

```shell
cd scenarios/ms-nonweb-plat/wasm-frontend-tvm/ && cargo build --release
cp ./target/release/wasm-frontend-tvm /usr/local/bin/
```

Check the usage of `cargo-frontend-tvm`:

```shell
~# wasm-frontend-tvm -h

Usage: wasm-frontend-tvm [options]

Options:
    -c, --ms-backend-config FILE_PATH
                        set wasm backend config file
    -o, --op-type VALUE set the operator type, currently ONLY support Add and
                        Sub, default: Add.
    -h, --help          print this help menu
```

## Appendix

### System packages install

* Rust (latest version)

    If you are running Windows, to install Rust, download and run the [RUST-INIT.EXE](https://win.rustup.rs/), and then follow the onscreen instructions.

    If you are a Linux user, run the following in your terminal, then follow the on-screen instructions to install Rust.

    ```shell
    curl https://sh.rustup.rs -sSf | sh
    ```

## Contribution

Lastly very thanks [@kazum](https://github.com/kazum) for having offered a lot of help when implementing this project.
