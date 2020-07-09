# MindSpore WebAssembly KernelCompiler with TVM Runtime

[![Releases](https://img.shields.io/github/release/leonwanghui/ms-backend-wasm/all.svg?style=flat-square)](https://github.com/leonwanghui/ms-backend-wasm/releases)
[![LICENSE](https://img.shields.io/github/license/leonwanghui/osc-serverless.svg?style=flat-square)](https://github.com/leonwanghui/ms-backend-wasm/blob/master/LICENSE)

#### Experimental notice: This project is still *experimental* and only serves as a proof of concept for running [MindSpore](https://github.com/mindspore-ai/mindspore) on [WebAssembly runtime](https://github.com/bytecodealliance/wasmtime) with [TVM stack](https://tvm.apache.org/).

- [MindSpore WebAssembly KernelCompiler with TVM Runtime](#mindspore-webassembly-kernelcompiler-with-tvm-runtime)
    - [Motivation](#motivation)
    - [Framework Landscape](#framework-landscape)
    - [Project Status](#project-status)
    - [PoC Guidelines](#poc-guidelines)
        - [Pre-installation](#pre-installation)
        - [Build wasm-kernelcompiler-tvm package](#build-wasm-kernelcompiler-tvm-package)
        - [Test](#test)
    - [Future Work](#future-work)
        - [Operator enhancement](#operator-enhancement)
        - [Performance benchmark](#performance-benchmark)
        - [Native TVM Rust runtime support](#native-tvm-rust-runtime-support)
    - [Appendix](#appendix)
        - [System packages install](#system-packages-install)
    - [Contribution](#contribution)

## Motivation

Currently the operator libs is `handwriting-only`, which is not flexible enough to scale out and not efficient to be executed. Therefore, we are working on adding [TVM runtime](https://github.com/apache/incubator-tvm) support in the short term.

<img src="https://github.com/dmlc/web-data/raw/master/tvm/tutorial/tvm_support_list.png" alt="TVM hardware support" width="600"/>

As demonstrated in TVM runtime [tutorials](https://tvm.apache.org/docs/tutorials/relay_quick_start.html), TVM already supports WASM as the optional hardware backend, so we can leverage the features of WebAssembly (portability, security) and TVM runtime (domain-specific, optimization) to build a flexible and auto-optimized backend for MindSpore.

## Framework Landscape

The figures below demonstrates the whole landscape of running MindSpore framework on WASM runtime with TVM compiler stack.

* WASM kernel compiler stack
    ```
                                                            _ _ _ _ _ _ _ _ _ _ _
                                                           |                     |
                                                           | TVM (TE) Python API |
                                                           |_ _ _ _ _ _ _ _ _ _ _|
                                                                     ||
                                                                     \/
                _ _ _ _ _ _ _ _ _ _ _ _                     _ _ _ _ _ _ _ _ _ _ _
               |                       |                   |                     |
               | WASM Kernel Compiler  |                   |  TVM Compiler Stack |
               |    (TVM runtime)      |                   |_ _ _ _ _ _ _ _ _ _ _|
               |_ _ _ _ _ _ _ _ _ _ _ _|                             ||
                          ||                                         \/
        _ _ _ _ _ _ _ _   ||   _ _ _ _ _ _ _ _ _ _ _            _ _ _ _ _ _ _
       |               |  \/  |                     |  llvm-ar |             |
       |  *.wasi.wasm  | <--- | libkernel_wasm32.a  | <------- | add.o sub.o |
       |_ _ _ _ _ _ _ _|      |_ _ _ _ _ _ _ _ _ _ _|          |_ _ _ _ _ _ _|
    ```

* WASM kernel runtime
    ```
     _ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _
    |                               |
    | MindSpore Frontend Expression |
    |_ _ _ _ _ _ _ _ _ _ _ _ _ _ _ _|
                  ||
                  \/
         _ _ _ _ _ _ _ _ _ _ _
        |                     |
        | WASM Kernel Runtime |
        |   (WASM runtime)    |
        |_ _ _ _ _ _ _ _ _ _ _|
                  ||
           _ _ _ _\/_ _ _ _
          |                |
          |  *.wasi.wasm   |
          |_ _ _ _ _ _ _ _ |
    ```

## Project Status

This project should be considered **experimental** at the very early stage, all rich features are under active development. Here is the current operator support matrix:

| Operator Name | Introduced | FP32 | INT32 | INT8 |
| ------------- | ---------- | ---- | ----- | ---- |
| Add | `v0.0.2` | ✔️ | <center>&mdash;</center> | <center>&mdash;</center> |
| Sub | `v0.0.2` | ✔️ | <center>&mdash;</center> | <center>&mdash;</center> |

**NOTICE**: Currently this project is ONLY tested on Ubuntu system, so `Ubuntu 16.04+` should be prepared as the testing environment.

## PoC Guidelines

### Pre-installation

* Rust

    Before running this demo, please make sure [Rust](#system-packages-install) has been installed.

    After Rust installed, execute the code below to add `wasm32-wasi` target:
    ```shell
    rustup target add wasm32-wasi
    cargo install cargo-wasi
    ```

* Wasmtime

    Please NOTICE that [Wasmtime](#system-packages-install) should be installed in advance.

* TVM

    Please follow TVM [installations](https://tvm.apache.org/docs/install/index.html), `export TVM_HOME=/path/to/tvm` and add `libtvm_runtime` to your `LD_LIBRARY_PATH`.

    *Note:* To run the end-to-end examples and tests, `tvm` and `topi` need to be added to your `PYTHONPATH` or it's automatic via an Anaconda environment when it is installed individually.

* LLVM

    `LLVM 10.0` or later is REQUIRED.

### Build wasm-kernelcompiler-tvm package

```shell
cd wasm-kernelruntime && cargo wasi build --release
cp ./target/wasm32-wasi/release/wasm_kernelcompiler_tvm.wasi.wasm ../wasm-kernelruntime/config/
```

### Test

You can run the command below to install the runtime package for testing (`rust` REQUIRED):
```shell
cd wasm-kernelruntime/ && cargo build --release
cp ./target/release/wasm-kernelruntime /usr/local/bin/
```

Check the usage of `wasm-kernelruntime`:

```shell
~# wasm-kernelruntime -h

Usage: wasm-kernelruntime [options]

Options:
    -c, --ms-backend-config FILE_PATH
                        set wasm backend config file
    -o, --op-type VALUE set the operator type, currently ONLY support Add and
                        Sub, default: Add.
    -h, --help          print this help menu
```

## Future Work

### Operator enhancement
TODO

### Performance benchmark
TODO

### Native TVM Rust runtime support
TODO

## Appendix

### System packages install

* Rust (latest version)

    If you are running Windows, to install Rust, download and run the [RUST-INIT.EXE](https://win.rustup.rs/), and then follow the onscreen instructions.

    If you are a Linux user, run the following in your terminal, then follow the on-screen instructions to install Rust.

    ```shell
    curl https://sh.rustup.rs -sSf | sh
    ```

* wasmtime (latest version)

    If you are running Windows 64-bit, download and run [Wasmtime Installer](https://github.com/CraneStation/wasmtime/releases/download/dev/wasmtime-dev-x86_64-windows.msi) then follow the onscreen instructions.

    If you're a Linux user run the following in your terminal, then follow the onscreen instructions to install `wasmtime`:

    ```shell
    curl https://wasmtime.dev/install.sh -sSf | bash
    ```

## Contribution

Lastly very thanks [@kazum](https://github.com/kazum) for having offered a lot of help when implementing this project.
