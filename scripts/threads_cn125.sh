#!/bin/sh

#SBATCH --account=csmpi
#SBATCH --partition=csmpi_long
#SBATCH --nodelist=cn125
#SBATCH --mem=0
#SBATCH --cpus-per-task=16
#SBATCH --time=10:00:00
#SBATCH --output=threads_cn125.out

./scripts/init_energy.sh
./scripts/make.sh

iter=20
out="threads_cn125"

echo "bench,dataset,threads,size,runtime,rapl" > results/"$out".csv

# Sweep thread-count only for 1e8 points
size=100000000
./scripts/vqhull.sh $size $iter $out
./scripts/pbbs.sh $size $iter $out
for threads in 2 4 8 16; do
  ./scripts/vqhull_par.sh $threads $size $iter $out
  ./scripts/pbbs_par.sh $threads $size $iter $out
done
