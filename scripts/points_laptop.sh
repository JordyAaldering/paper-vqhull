#!/bin/sh

./scripts/init_energy.sh
./scripts/make.sh

iter=20
out="points_laptop"

#echo "bench,dataset,threads,size,runtime,rapl,core,uncore,psys,gpu" > res/"$out".csv

# Sweep inputs 1e5--1e8 only for 12 threads
threads=12
#for size in 100000 1000000 10000000 100000000; do
#  ./scripts/vqhull_par.sh $threads $size $iter $out numactl -C 0-11
#  ./scripts/pbbs_par.sh $threads $size $iter $out numactl -C 0-11
#done

# Sweep 1e9. Because of limited disk space, we generate and delete the input
# for one benchmark at a time.

(
cd pbbsbench || exit 1
make geometryData
)

(
cd code/examples || exit 1
make text2bin
)

gen_input()
{
    name="$1"
    option="$2"

    ./pbbsbench/testData/geometryData/randPoints "$option" -d 2 $size \
        data/"$name"_"$size".in
    ./code/examples/text2bin < data/"$name"_"$size".in > data/"$name"_"$size".bin
}

bench_vqhull()
{
    name=$1
    size=$2
    echo "VQhull ($threads) $name $size"
    {
        i=1
        while [ $i -le $iter ]
        do
            OMP_NUM_THREADS=$threads OMP_PLACES=cores numactl -C 0-11 \
                ./code/examples/test_quickhull_par m b < data/"$name"_"$size".bin
            i=$((i + 1))
        done
    } | awk -v name=$name -v threads=$threads -v size=$size '{
        printf "VQhull,%s,%d,%d,%s\n", name, threads, size, $0;
    }' >> res/"$out".csv
}

bench_pbbs()
{
    name=$1
    echo "PBBS ($threads) $name $size"
    PARLAY_NUM_THREADS=$threads numactl -C 0-11 \
        pbbsbench/benchmarks/convexHull/quickHull/hull -r $iter data/"$1"_"$size".in \
        | sed 's/Parlay time: //g' \
        | sed '/^$/d' \
        | awk -v name=$name -v threads=$threads -v size=$size '{
            printf "PBBS,%s,%d,%d,%s\n", name, threads, size, $0;
        }' >> res/"$out".csv
}

bench_all()
{
    name="$1"
    option="$2"
    size="$3"

    gen_input "$name" "$option"
    bench_vqhull "$name" "$size"
    bench_pbbs "$name" "$size"

    rm data/"$name"_"$size".in
    rm data/"$name"_"$size".bin
}

large_size=1000000000
bench_all "disk" "-s" "$large_size"
bench_all "circle" "-S" "$large_size"
bench_all "kuzmin" "-p" "$large_size"
