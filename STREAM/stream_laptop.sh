#!/bin/sh

make clean
make stream_c
OMP_PLACES=cores OMP_NUM_THREADS=12 numactl -C 0-11 ./stream_c
