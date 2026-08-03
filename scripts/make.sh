#!/bin/sh

(
cd pbbsbench/benchmarks/convexHull/serialHull || exit
make clean
make
)

(
cd pbbsbench/benchmarks/convexHull/quickHull || exit
make clean
make
)

(
cd code || exit
make clean
make uninstall
make install
)

(
cd code/examples || exit
make clean
make test_quickhull
make test_quickhull_par
)
