#!/usr/bin/env python3
# Licensed to the Apache Software Foundation (ASF) under one
# or more contributor license agreements.  See the NOTICE file
# distributed with this work for additional information
# regarding copyright ownership.  The ASF licenses this file
# to you under the Apache License, Version 2.0 (the
# "License"); you may not use this file except in compliance
# with the License.  You may obtain a copy of the License at
#
#   http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing,
# software distributed under the License is distributed on an
# "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
# KIND, either express or implied.  See the License for the
# specific language governing permissions and limitations
# under the License.

"""Prepares a simple TVM library for operators."""
import argparse
import os
import subprocess
import sys

import tvm
from tvm import te

G_TARGET = "llvm -target=wasm32-unknown-unknown -mattr=+simd128 --system-lib"


def _build_add_op(obj_file, name='add'):
    n = te.var('n')
    A = te.placeholder((n,), name='A')
    B = te.placeholder((n,), name='B')
    C = te.compute(A.shape, lambda *i: A(*i) + B(*i), name='C')
    s = tvm.te.create_schedule(C.op)
    s[C].parallel(s[C].op.axis[0])
    m = tvm.lower(s, [A, B, C], name=name, simple_mode=True)
    tvm.build(m, target=G_TARGET).save(obj_file)


def _build_sub_op(obj_file, name='sub'):
    n = te.var('n')
    A = te.placeholder((n,), name='A')
    B = te.placeholder((n,), name='B')
    C = te.compute(A.shape, lambda *i: A(*i) - B(*i), name='C')
    s = tvm.te.create_schedule(C.op)
    s[C].parallel(s[C].op.axis[0])
    m = tvm.lower(s, [A, B, C], name=name, simple_mode=True)
    tvm.build(m, target=G_TARGET).save(obj_file)


def build_ops_lib(out_dir):
    """Compiles the operators with TVM"""
    add_obj_file = os.path.join(out_dir, 'add.o')
    sub_obj_file = os.path.join(out_dir, 'sub.o')
    _build_add_op(add_obj_file)
    _build_sub_op(sub_obj_file)

    # Run llvm-ar to archive obj_file into lib_file
    lib_file = os.path.join(out_dir, 'libops_wasm32.a')
    cmds = [os.environ.get("LLVM_AR", "llvm-ar-10"), 'rcs',
            lib_file,
            add_obj_file,
            sub_obj_file,
            ]
    subprocess.run(cmds)


if __name__ == '__main__':
    out_dir = os.path.join(sys.path[0], "../lib")
    if not os.path.exists(out_dir):
        os.makedirs(out_dir)

    build_ops_lib(out_dir)
