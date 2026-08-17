#!/bin/sh

#SBATCH --account=csmpi
#SBATCH --partition=csmpi_fpga_long
#SBATCH --nodelist=cn132
#SBATCH --mem=0
#SBATCH --cpus-per-task=32
#SBATCH --time=10:00:00
#SBATCH --output=points_cn132.out

./scripts/init_energy.sh
./scripts/make.sh

iter=20
out="points_cn132"

echo "bench,dataset,threads,size,runtime,rapl,rapl2" > res/"$out".csv

# Sweep inputs 1e5--1e9 only for 32 threads
threads=32
for size in 100000 1000000 10000000 100000000 1000000000; do
  OMP_PROC_BIND=spread ./scripts/vqhull_par.sh $threads $size $iter $out numactl --interleave all
  ./scripts/pbbs_par.sh $threads $size $iter $out numactl --interleave all
done
