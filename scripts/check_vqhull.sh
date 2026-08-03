#!/bin/sh

#SBATCH --account=csmpi
#SBATCH --partition=csmpi_long
#SBATCH --mem=0
#SBATCH --nodelist=cn125
#SBATCH --cpus-per-task=16
#SBATCH --time=1:00:00
#SBATCH --output=check_vqhull.out

if [ "$#" -ne 1 ]; then
    printf 'Usage: %s SIZE\n' "$0" >&2
    printf '\tSIZE: number of points\n' >&2
    exit 1
fi

n="$1"

(
cd code || exit
make clean
make uninstall
make clean
BUILD=DEBUG make install
)

# Crashes for some GCC version
export ASAN_OPTIONS=detect_leaks=0

(
cd code/examples || exit
make clean
BUILD=DEBUG make test_quickhull index2points
)

(
cd pbbsbench/benchmarks/convexHull/serialHull || exit
make
)

check()
{
    name="$1"_"$n"

    echo ""
    echo "=============== $name ==============="

    ./code/examples/test_quickhull p b < data/"$name".bin > data/"$name".quickhull.out
    ./pbbsbench/benchmarks/convexHull/serialHull/hull \
        -o data/"$name".pbbs_serial_indices.out \
        data/"$name".in
    ./code/examples/index2points data/"$name".in \
        data/"$name".pbbs_serial_indices.out > \
        data/"$name".pbbs_serial.out

    if diff data/"$name".pbbs_serial.out data/"$name".quickhull.out; then
        echo "Same result as pbbs on $name"
    fi

    echo "=============== $name ==============="
}

check disk
check circle
check kuzmin
