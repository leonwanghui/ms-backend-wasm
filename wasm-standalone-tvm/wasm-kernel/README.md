<!--- Licensed to the Apache Software Foundation (ASF) under one -->
<!--- or more contributor license agreements.  See the NOTICE file -->
<!--- distributed with this work for additional information -->
<!--- regarding copyright ownership.  The ASF licenses this file -->
<!--- to you under the Apache License, Version 2.0 (the -->
<!--- "License"); you may not use this file except in compliance -->
<!--- with the License.  You may obtain a copy of the License at -->

<!---   http://www.apache.org/licenses/LICENSE-2.0 -->

<!--- Unless required by applicable law or agreed to in writing, -->
<!--- software distributed under the License is distributed on an -->
<!--- "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY -->
<!--- KIND, either express or implied.  See the License for the -->
<!--- specific language governing permissions and limitations -->
<!--- under the License. -->

# WebAssembly Kernel for Deep Learning Framework with TVM Runtime

[![Releases](https://img.shields.io/github/release/leonwanghui/ms-backend-wasm/all.svg?style=flat-square)](https://github.com/leonwanghui/ms-backend-wasm/releases)
[![LICENSE](https://img.shields.io/github/license/leonwanghui/osc-serverless.svg?style=flat-square)](https://github.com/leonwanghui/ms-backend-wasm/blob/master/LICENSE)

#### Experimental notice: This project is still *experimental* and only serves as a proof of concept for running deep learning frameworks on [WebAssembly runtime](https://github.com/bytecodealliance/wasmtime) with [TVM stack](https://tvm.apache.org/).

- [WebAssembly Kernel for Deep Learning Framework with TVM Runtime](#webassembly-kernel-for-deep-learning-framework-with-tvm-runtime)
    - [Motivation](#motivation)
    - [Framework Landscape](#framework-landscape)
    - [Project Status](#project-status)
    - [PoC Guidelines](#poc-guidelines)
        - [Pre-installation](#pre-installation)
        - [Build wasm-kernel package](#build-wasm-kernel-package)
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

* WASM kernel generation
    ```
                                                          _ _ _ _ _ _ _ _ _ _ _
                                                         |                     |
                                                         | TVM (TE) Python API |
                                                         |_ _ _ _ _ _ _ _ _ _ _|
                                                                   ||
                                                                   \/
              _ _ _ _ _ _ _ _ _ _ _ _                     _ _ _ _ _ _ _ _ _ _ _
             |                       |                   |                     |
             |  WASM Kernel AppCode  |                   |  TVM Compiler Stack |
             |     (TVM runtime)     |                   |_ _ _ _ _ _ _ _ _ _ _|
             |_ _ _ _ _ _ _ _ _ _ _ _|                             ||
                        ||                                         \/
      _ _ _ _ _ _ _ _   ||   _ _ _ _ _ _ _ _ _ _ _            _ _ _ _ _ _ _
     |               |  \/  |                     |  llvm-ar |             |
     | wasm_ops.wasm | <--- |   libops_wasm32.a   | <------- | add.o sub.o |
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
           _ _ _ _\/_ _ _ _ _
          |                 |
          |  wasm_ops.wasm  |
          |_ _ _ _ _ _ _ _ _|
    ```

## Project Status

This project should be considered **experimental** at the very early stage, all rich features are under active development. Here is the current operator support matrix:

| Operator Name | FP32 | INT32 | INT8 |
| ------------- | ---- | ----- | ---- |
| Add | ✔️ | <center>&mdash;</center> | <center>&mdash;</center> |
| Sub | ✔️ | <center>&mdash;</center> | <center>&mdash;</center> |

**NOTICE**: Currently this project is ONLY tested on Ubuntu system, so `Ubuntu 16.04+` should be prepared as the testing environment.

## PoC Guidelines

### Pre-installation

* Rust

    Before running this demo, please make sure [Rust](#system-packages-install) has been installed.

    After Rust installed, execute the code below to add `wasm32-wasi` target:
    ```shell
    rustup target add wasm32-wasi
    ```

* TVM

    Please follow TVM [installations](https://tvm.apache.org/docs/install/index.html) for the detailed instruction.

* LLVM

    `LLVM 10.0` or later is REQUIRED.

### Build wasm-kernel package

```shell
cd wasm-kernel && python ./tools/build_ops_lib.py
cargo build --release
cp ./target/wasm32-wasi/release/wasm_kernel.wasm ../wasm-kernelruntime/config/
```

## Future Work

### Operator enhancement
TODO

### Performance benchmark

We are working on several improvements on performances:
* WebAssembly SIMD128 support (**Done**)
* Operator fusion from MindSpore frontend

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

## Contribution

Lastly very thanks [@kazum](https://github.com/kazum) for having offered a lot of help when implementing this project.
