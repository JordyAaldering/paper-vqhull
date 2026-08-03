#!/bin/sh

#SBATCH --account=csmpi
#SBATCH --partition=csmpi_long
#SBATCH --mem=0
#SBATCH --nodelist=cn125
#SBATCH --cpus-per-task=8
#SBATCH --time=1:00:00
#SBATCH --output=gen_input.out

if [ "$#" -ne 1 ]; then
    printf 'Usage: %s SIZE\n' "$0" >&2
    printf '\tSIZE: number of points\n' >&2
    exit 1
fi

size=$1

(
cd pbbsbench || exit 1
make geometryData
)

./pbbsbench/testData/geometryData/randPoints -s -d 2 $size data/disk_"$size".in
./pbbsbench/testData/geometryData/randPoints -S -d 2 $size data/circle_"$size".in
./pbbsbench/testData/geometryData/randPoints -p -d 2 $size data/kuzmin_"$size".in

(
cd code/examples || exit 1
make text2bin
)

./code/examples/text2bin < data/disk_"$size".in > data/disk_"$size".bin
./code/examples/text2bin < data/circle_"$size".in > data/circle_"$size".bin
./code/examples/text2bin < data/kuzmin_"$size".in > data/kuzmin_"$size".bin
