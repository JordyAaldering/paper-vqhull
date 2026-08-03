#!/bin/sh

if [ "$#" -lt 3 ]; then
    printf 'Usage: %s SIZE ITER OUT\n' "$0" >&2
    printf '\tSIZE:  number of points\n' >&2
    printf '\tITER:  number of times to repeat the experiment\n' >&2
    printf '\tOUT:   output file name\n' >&2
    printf '\tEXTRA: Extra stuff to preprend to command\n' >&2
    exit 1
fi

size=$1
iter=$2
out=$3
shift 3
environment=$@

bench()
{
    name=$1
    echo "VQhull seq $name $size"
    {
        i=1
        while [ $i -le $iter ]
        do
            $environment ./code/examples/test_quickhull m b < data/"$name"_"$size".bin
            i=$((i + 1))
        done
    } | awk -v name="$name" -v size="$size" '{
        printf "VQhull,%s,1,%d,%s\n", name, size, $0;
    }' >> "results/"$out".csv"
}

bench circle
bench disk
bench kuzmin
