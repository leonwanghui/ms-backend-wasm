# MindSpore WebAssembly GraphCompiler with TVM Runtime

[![Releases](https://img.shields.io/github/release/leonwanghui/ms-backend-wasm/all.svg?style=flat-square)](https://github.com/leonwanghui/ms-backend-wasm/releases)
[![LICENSE](https://img.shields.io/github/license/leonwanghui/osc-serverless.svg?style=flat-square)](https://github.com/leonwanghui/ms-backend-wasm/blob/master/LICENSE)

#### Experimental notice: This project is still *experimental* and only serves as a proof of concept for running [MindSpore](https://github.com/mindspore-ai/mindspore) on [WebAssembly runtime](https://github.com/bytecodealliance/wasmtime) with [TVM stack](https://tvm.apache.org/).

- [MindSpore WebAssembly GraphCompiler with TVM Runtime](#mindspore-webassembly-graphcompiler-with-tvm-runtime)
    - [Motivation](#motivation)
    - [Framework Landscape](#framework-landscape)
    - [Project Status](#project-status)
    - [PoC Guidelines](#poc-guidelines)
        - [Pre-installation](#pre-installation)
        - [Build ResNet50 model](#build-resnet50-model)
        - [Build wasm-graphcompiler-tvm package](#build-wasm-graphcompiler-tvm-package)
        - [Test](#test)
    - [Future Work](#future-work)
        - [More networks support](#more-networks-support)
        - [Performance benchmark](#performance-benchmark)
        - [Native TVM Rust runtime support](#native-tvm-rust-runtime-support)
    - [Appendix](#appendix)
        - [System packages install](#system-packages-install)
    - [Contribution](#contribution)

## Motivation

After finishing [wasm-kernelcompiler-tvm](../wasm-kernelcompiler-tvm/README.md) MVP, we found that directly generating generic kernels may not give the best performance for deployment, as the compilation approach takes a lot of shape specialization and fusions. Therefore, we are working on adding [TVM Relay](https://tvm.apache.org/docs/dev/relay_intro.html) support in the short term.

## Framework Landscape

The figures below demonstrate the whole landscape of running MindSpore model on WASM runtime with TVM compiler stack.

* WASM graph compiler stack
    ```
       _ _ _ _ _ _ _ _ _ _        _ _ _ _ _ _ _        _ _ _ _ _ _ _ _ _ _ _ _
      |                   |      |             |      |                       |
      |  MindSpore Model  | ---> |  ONNX Model | ---> |  TVM Relay Python API |
      |_ _ _ _ _ _ _ _ _ _|      |_ _ _ _ _ _ _|      |_ _ _ _ _ _ _ _ _ _ _ _|
                                                                 ||
                                                                 \/
                 _ _ _ _ _ _ _ _ _ _ _                  _ _ _ _ _ _ _ _ _ _ _
                |                     |                |                     |
                | WASM Graph Compiler |                |  TVM Compiler Stack |
                |    (TVM runtime)    |                |_ _ _ _ _ _ _ _ _ _ _|
                |_ _ _ _ _ _ _ _ _ _ _|                          ||
                          ||                                     \/
        _ _ _ _ _ _ _ _   ||   _ _ _ _ _ _ _ _ _ _            _ _ _ _ _
       |               |  \/  |                   |  llvm-ar |         |
       |  *.wasi.wasm  | <--- | libgraph_wasm32.a | <------- | graph.o |
       |_ _ _ _ _ _ _ _|      |_ _ _ _ _ _ _ _ _ _|          |_ _ _ _ _|
    ```

* WASM graph runtime
    ```
         _ _ _ _ _ _ _ _ _ _ _
        |                     |
        | WASM Graph Runtime  |
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

| Model Name | Status |
| ---------- | ------ |
| ResNet50 | ✔️ |
| LeNet | <center>&mdash;</center> |

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

### Build ResNet50 model

- Build DL library in the WebAssembly format.

  - Download model

    ```
    cd wasm-graphcompiler/tools && wget https://s3.amazonaws.com/onnx-model-zoo/resnet/resnet50v1/resnet50v1.onnx
    ```

  - Compile

    ```
    LLVM_AR=llvm-ar-10 python ./build_grpah_lib.py -O3 ./resnet50v1.onnx
    ```

### Build wasm-graphcompiler-tvm package

```shell
cd wasm-graphcompiler && cargo wasi build --release
cp ./target/wasm32-wasi/release/wasm_graphcompiler_tvm.wasi.wasm ../wasm-graphruntime/tools/
```

### Test

You can run the command below to install the runtime package for testing (`rust` REQUIRED):
```shell
cd wasm-graphruntime/ && cargo build --release
cp ./target/release/wasm-graphruntime /usr/local/bin/
```

Check the usage of `wasm-graphruntime`:

```shell
~# wasm-graphruntime -h

Usage: wasm-graphruntime [options]

Options:
    -c, --ms-backend-config FILE_PATH
                        set wasm backend config file
    -i, --input-data-file FILE_PATH
                        set the path to input image file
    -h, --help          print this help menu
```

Next perform model inference using these commands below:
```
$ cd wasm-graphruntime/tools && wget -O cat.png https://github.com/dmlc/mxnet.js/blob/master/data/cat.png?raw=true
$ wasm-graphruntime -c ./wasm_graphcompiler_tvm.wasi.wasm -i ./cat.png
original image dimensions: (256, 256)
resized image dimensions: (224, 224)
input image belongs to the class `tabby, tabby cat`
```

## Future Work

### More networks support
TODO

### Performance benchmark

We are working on several improvements on performances:
* WebAssemvly SIMD support
* Auto-tvm enhancement for llvm target

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
