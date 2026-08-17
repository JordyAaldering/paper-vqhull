#!/bin/sh

./scripts/init_energy.sh
./scripts/make.sh

iter=20
out="threads_laptop"

echo "bench,dataset,threads,size,runtime,rapl,core,uncore,psys,gpu" > res/"$out".csv

# Sweep thread-count only for 1e8 points
size=100000000
./scripts/vqhull.sh $size $iter $out numactl -C 0-11
./scripts/pbbs.sh $size $iter $out numactl -C 0-11
for threads in 2 4 6 12; do
    ./scripts/vqhull_par.sh $threads $size $iter $out numactl -C 0-11
    ./scripts/pbbs_par.sh $threads $size $iter $out numactl -C 0-11
done
