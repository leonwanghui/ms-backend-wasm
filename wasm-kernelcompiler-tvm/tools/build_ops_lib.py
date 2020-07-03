#!/usr/bin/env python3
"""Prepares a simple TVM library for operators."""

from os import path as osp
import sys

import tvm
from tvm import te


def add():
    n = te.var('n')
    A = te.placeholder((n,), name='A')
    B = te.placeholder((n,), name='B')
    C = te.compute(A.shape, lambda *i: A(*i) + B(*i), name='C')
    s = tvm.te.create_schedule(C.op)
    s[C].parallel(s[C].op.axis[0])
    m = tvm.lower(s, [A, B, C], name="add", simple_mode=True)
    tvm.build(m, target="llvm -target=wasm32-unknown-unknown --system-lib").save(osp.join(sys.argv[1], 'add.o'))


def sub():
    n = te.var('n')
    A = te.placeholder((n,), name='A')
    B = te.placeholder((n,), name='B')
    C = te.compute(A.shape, lambda *i: A(*i) - B(*i), name='C')
    s = tvm.te.create_schedule(C.op)
    s[C].parallel(s[C].op.axis[0])
    m = tvm.lower(s, [A, B, C], name="sub", simple_mode=True)
    tvm.build(m, target="llvm -target=wasm32-unknown-unknown --system-lib").save(osp.join(sys.argv[1], 'sub.o'))


if __name__ == '__main__':
    add()
    sub()
