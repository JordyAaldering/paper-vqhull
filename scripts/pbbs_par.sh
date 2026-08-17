#!/bin/sh

if [ "$#" -lt 4 ]; then
    printf 'Usage: %s THREADS SIZE ITER OUT EXTRA\n' "$0" >&2
    printf '\tTHREADS:      number of threads to use\n' >&2
    printf '\tSIZE:         number of points\n' >&2
    printf '\tITER:         number of times to repeat the experiment\n' >&2
    printf '\tOUT:          output file name\n' >&2
    printf '\tENVIRONMENT:  extra environment / commands before measurement\n' >&2
    exit 1
fi

threads=$1
size=$2
iter=$3
out=$4
shift 4
environment=$@

bench()
{
    name=$1
    echo "PBBS ($threads) $name $size"
    PARLAY_NUM_THREADS=$threads $environment \
        pbbsbench/benchmarks/convexHull/quickHull/hull -r $iter data/"$1"_"$size".in \
        | sed 's/Parlay time: //g' \
        | sed '/^$/d' \
        | awk -v name=$name -v threads=$threads -v size=$size '{
            printf "PBBS,%s,%d,%d,%s\n", name, threads, size, $0;
        }' >> "res/"$out".csv"
}

bench circle
bench disk
bench kuzmin
