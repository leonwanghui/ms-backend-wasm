# MindSpore WebAssembly GraphCompiler with TVM Runtime

[![Releases](https://img.shields.io/github/release/leonwanghui/ms-backend-wasm/all.svg?style=flat-square)](https://github.com/leonwanghui/ms-backend-wasm/releases)
[![LICENSE](https://img.shields.io/github/license/leonwanghui/osc-serverless.svg?style=flat-square)](https://github.com/leonwanghui/ms-backend-wasm/blob/master/LICENSE)

#### Experimental notice: This project is still *experimental* and only serves as a proof of concept for running [MindSpore](https://github.com/mindspore-ai/mindspore) on [WebAssembly runtime](https://github.com/bytecodealliance/wasmtime) with [TVM stack](https://tvm.apache.org/).

- [MindSpore WebAssembly GraphCompiler with TVM Runtime](#mindspore-webassembly-graphcompiler-with-tvm-runtime)
    - [Motivation](#motivation)
    - [Framework Landscape](#framework-landscape)
    - [Project Status](#project-status)
    - [PoC Guidelines](#poc-guidelines)
    - [Appendix](#appendix)
        - [System packages install](#system-packages-install)
    - [Contribution](#contribution)

## Motivation

After finishing [wasm-kernelcompiler-tvm](../wasm-kernelcompiler-tvm/README.md) MVP, we found that directly generating generic kernels may not give the best performance for deployment, as the compilation approach takes a lot of shape specialization and fusions. Therefore, we are working on adding [TVM Relay](https://tvm.apache.org/docs/dev/relay_intro.html) support in the short term.

## Framework Landscape

The figure below demonstrates the whole landscape of running MindSpore framework on WASM runtime with TVM compiler stack.
```
       _ _ _ _ _ _ _ _ _ _        _ _ _ _ _ _ _        _ _ _ _ _ _ _ _ _ _ _ _
      |                   |      |             |      |                       |
      |  MindSpore Model  | ---> |  ONNX Model | ---> |  TVM Relay Python API |
      |_ _ _ _ _ _ _ _ _ _|      |_ _ _ _ _ _ _|      |_ _ _ _ _ _ _ _ _ _ _ _|
                                                                 ||
   _ _ _ _ _ _ _ _ _ _ _ _ _ _                                   \/
  |                           |                         _ _ _ _ _ _ _ _ _ _ _
  |  MindSpore WASM Backend   |                        |                     |
  |      (WASM runtime)       |                        |  TVM Compiler Stack |
  |_ _ _ _ _ _ _ _ _ _ _ _ _ _|                        |_ _ _ _ _ _ _ _ _ _ _|
              ||                                                 ||
              \/                                                 \/
        _ _ _ _ _ _ _ _        _ _ _ _ _ _ _ _ _ _            _ _ _ _ _
       |               |      |                   |  llvm-ar |         |
       |  TVM Runtime  | <--- | libgraph_wasm32.a | <------- | graph.o |
       |_ _ _ _ _ _ _ _|      |_ _ _ _ _ _ _ _ _ _|          |_ _ _ _ _|
```

## Project Status

This project should be considered **experimental** at the very early stage, all rich features are under active development. Here is the current operator support matrix:

| Model Name | Status |
| ---------- | ------ |
| ResNet50 | ✔️ |
| LeNet | <center>&mdash;</center> |

**NOTICE**: Currently this project is ONLY tested on Ubuntu system, so `Ubuntu 16.04+` should be prepared as the testing environment.

## PoC Guidelines

Before running this demo, please make sure [`Rust`](#system-packages-install) has been installed.

Next run the command below to install the frontend package for testing (`rust` REQUIRED):

```shell
cd scenarios/ms-nonweb-plat/wasm-graphfrontend/ && cargo build --release
cp ./target/release/wasm-graphfrontend /usr/local/bin/
```

Check the usage of `wasm-graphfrontend`:

```shell
~# wasm-graphfrontend -h

Usage: wasm-graphfrontend [options]

Options:
    -c, --ms-backend-config FILE_PATH
                        set wasm backend config file
    -i, --input-data-file FILE_PATH
                        set the path to input image file
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
