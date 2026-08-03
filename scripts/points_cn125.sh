#!/bin/sh

#SBATCH --account=csmpi
#SBATCH --partition=csmpi_long
#SBATCH --nodelist=cn125
#SBATCH --mem=0
#SBATCH --cpus-per-task=16
#SBATCH --time=10:00:00
#SBATCH --output=points_cn125.out

./scripts/init_energy.sh
./scripts/make.sh

iter=20
out="points_cn125"

echo "bench,dataset,threads,size,runtime,rapl" > results/"$out".csv

# Sweep inputs 1e5--1e9 only for 16 threads
threads=16
for size in 100000 1000000 10000000 100000000 1000000000; do
  ./scripts/vqhull_par.sh $threads $size $iter $out
  ./scripts/pbbs_par.sh $threads $size $iter $out
done
