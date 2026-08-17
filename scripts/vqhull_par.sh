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
    echo "VQhull ($threads) $name $size"
    {
        i=1
        while [ $i -le $iter ]
        do
            OMP_NUM_THREADS=$threads OMP_PLACES=cores $environment \
                ./code/examples/test_quickhull_par m b < data/"$name"_"$size".bin
            i=$((i + 1))
        done
    } | awk -v name=$name -v threads=$threads -v size=$size '{
        printf "VQhull,%s,%d,%d,%s\n", name, threads, size, $0;
    }' >> "res/"$out".csv"
}

bench circle
bench disk
bench kuzmin
