# MindSpore WebAssembly Backend

[![Releases](https://img.shields.io/github/release/leonwanghui/ms-backend-wasm/all.svg?style=flat-square)](https://github.com/leonwanghui/ms-backend-wasm/releases)
[![LICENSE](https://img.shields.io/github/license/leonwanghui/osc-serverless.svg?style=flat-square)](https://github.com/leonwanghui/ms-backend-wasm/blob/master/LICENSE)

#### Experimental notice: This project is still *experimental* and only serves as a proof of concept for running [MindSpore](https://github.com/mindspore-ai/mindspore) on WebAssembly runtime.

- [MindSpore WebAssembly Backend](#mindspore-webassembly-backend)
    - [Background](#background)
        - [WebAssembly and WASI introduction](#webassembly-and-wasi-introduction)
        - [New ML framework backend with WASI](#new-ml-framework-backend-with-wasi)
        - [Case study - TensorFlow WebAssembly support](#case-study---tensorflow-webassembly-support)
    - [Project Status](#project-status)
    - [Use cases](#use-cases)
        - [Web scenarios](#web-scenarios)
        - [Non-web scenarios](#non-web-scenarios)
    - [Future Work](#future-work)
        - [TVM runtime support](#tvm-runtime-support)
        - [TOPI WASM build](#topi-wasm-build)
        - [MindSpore frontend integration](#mindSpore-frontend-integration)
        - [Wasmtime interface types support](#wasmtime-interface-types-support)
    - [Appendix](#appendix)
        - [System packages install](#system-packages-install)

## Background

### WebAssembly and WASI introduction

[WebAssembly](https://webassembly.org/) was proposed to help address the javascript code execution performance problem. WASM has now became a widely used and de-facto runtime standard in web development, especially for mobile applications.

[WASI](http://wasi.dev/) is a modular system interface for WebAssembly. As described in this [blogpost](https://hacks.mozilla.org/2019/03/standardizing-wasi-a-webassembly-system-interface/), WebAssembly is an assembly language for a conceptual machine, so it needs a system interface for a conceptual operating system, not any single operating system. This way, it can be run across all different OSs.

### New ML framework backend with WASI

We believe that with more maturity at WASI, it is possible that we could have a general backend operator library that could work across all scenarios (Cloud/Edge/Mobile). Together with a WASM port of [MindSpore](https://www.mindspore.cn/), our newly open sourced all scenario deep learning framework, WASI could enable a new backend-agnostic, highly secure and performant stack that help user and developers alike to be able to develop new AI applications with better portability.

WASM could also bring innovation to AI technologies like [Federated Learning](https://en.wikipedia.org/wiki/Federated_learning). Unlike the conventional container based deployment for federated learning applications, WASM based solution could bring good isolation, small memory consumption and therefore making the MPC more efficient and secure.

### Case study - TensorFlow WebAssembly support

TensorFlow community recently released a [blogpost](https://blog.tensorflow.org/2020/03/introducing-webassembly-backend-for-tensorflow-js.html) on Mar 11th that TF can now support a [WebAssembly](https://webassembly.org/) (WASM) backend for TensorFlow.js. TensorFlow WASM backend provides a new choice for the user to directly run inference on mobile CPU. It also provides a good combination of performance enhancement and portability. The execution speed is 2–10 times faster than javascript and the support for mobile is better than WebGL. In light of the recent development of [SIMD support](https://github.com/WebAssembly/simd) in WASM community, the inference performance could be further enhanced.

## Project Status

This project should be considered **experimental** at the very early stage, all rich features are under active development. Here is the current operator support matrix:

| Operator Name | Introduced | FP32 | INT32 | INT8 |
| ------------- | ---------- | ---- | ----- | ---- |
| Add | `v0.0.1` | ✔️ | ✔️ | ✔️ |
| Mul | `v0.0.1` | ✔️ | ✔️ | ✔️ |
| Argmax | `v0.0.1` | ✔️ | ✔️ | ✔️ |
| EqualCount | `v0.0.1` | ✔️ | ✔️ | ✔️ |

**NOTICE**: Currently this project is ONLY tested on Ubuntu system, so `Ubuntu 16.04+` should be prepared as the testing environment.

## Use cases

### Web scenarios

If you want to utilize the `ms-backend-wasm` package in web browser, please make sure [`Node.js`](#system-packages-install) has been installed.

Next run the command below to install the package (`npm` REQUIRED):

```shell
cd scenarios/ms-web-plat/ && sudo npm i
sudo npm run build
sudo npm run serve
```

Then open the browser and login to `http://{ your_host_ip }:8088` to access the demo.

### Non-web scenarios

Before running `ms-backend-wasm` package in non-web scenarios, please make sure [`Rust`](#system-packages-install) has been installed.

Next run the command below to install the package (`rust` REQUIRED):

```shell
cd scenarios/ms-nonweb-plat/wasm-frontend/ && cargo run build --release
cp ./target/release/wasm-frontend /usr/local/bin/
```

Check the usage of `cargo-frontend`:

```shell
~# wasm-frontend -h

Usage: wasm-frontend [options]

Options:
    -c, --ms-backend-config FILE_PATH
                        set wasm backend config file
    -o, --op-type VALUE set the operator type, ONLY supports Add, Mul, Argmax
                        and EqualCount, default: Add.
    -d, --data-type VALUE
                        set the data type, ONLY supports FP32, INT32 and INT8,
                        default: FP32.
    -I, --input VALUE   set the input data
    -i, --input-data-file FILE_PATH
                        set input data file
    -h, --help          print this help menu
```

## Future Work

### TVM runtime support

Currently the operator libs is `handwriting-only`, which is not flexible enough to scale out and not efficient to be executed. Therefore, we are working on adding [TVM runtime](https://github.com/apache/incubator-tvm) support in the short term.

<img src="https://github.com/dmlc/web-data/raw/master/tvm/tutorial/tvm_support_list.png" alt="TVM hardware support" width="600"/>

As demonstrated in TVM runtime [tutorials](https://tvm.apache.org/docs/tutorials/relay_quick_start.html), TVM already supports WASM as the optional hardware backend, so we can leverage the features of WebAssembly (portability, security) and TVM runtime (domain-specific, optimization) to build a flexible and auto-optimized backend for MindSpore.

### TOPI WASM build

[TOPI](https://github.com/apache/incubator-tvm/tree/master/topi) (TVM Operator Inventory) provides numpy-style generic operations and schedules with higher abstractions than TVM, so it's highly required to add the topi package compilation with WASM backend support in TVM community.

### MindSpore frontend integration

Although it is ONLY a PoC on running MindSpore on WebAssembly runtime, we will drive fast iteration to release the integration to MindSpore frontend in later versions.

### Wasmtime interface types support

With the latest stable version (`v0.16.0`), support for interface types has temporarily removed from Wasmtime. So currently working with WebAssembly modules means it can only deal with integers and floats, and more rich types (like byte arrays, strings, structure, etc.) are not supported. For more information see https://github.com/bytecodealliance/wasmtime/issues/677.

## Appendix

### System packages install

* Rust (latest version)

    If you are running Windows, to install Rust, download and run the [RUST-INIT.EXE](https://win.rustup.rs/), and then follow the onscreen instructions.

    If you are a Linux user, run the following in your terminal, then follow the on-screen instructions to install Rust.

    ```shell
    curl https://sh.rustup.rs -sSf | sh
    ```

* Node.js (latest version)

    ```shell
    sudo apt-get install -y npm
    sudo npm install n -g
    sudo n stable

    # Check the version of npm and node
    npm -v
    node -v
    ```

* wasmtime (for developers)

    If you are running Windows 64-bit, download and run [Wasmtime Installer](https://github.com/CraneStation/wasmtime/releases/download/dev/wasmtime-dev-x86_64-windows.msi) then follow the onscreen instructions.

    If you're a Linux user run the following in your terminal, then follow the onscreen instructions to install `wasmtime`:

    ```shell
    curl https://wasmtime.dev/install.sh -sSf | bash
    ```

* wasm-pack (for developers)

    If you are running Windows 64-bit, download and run [wasm-pack-init.exe](https://github.com/rustwasm/wasm-pack/releases/download/v0.9.1/wasm-pack-init.exe) then follow the onscreen instructions.

    If you're a Linux user run the following in your terminal, then follow the onscreen instructions to install `wasm-pack`:

    ```shell
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
    ```
