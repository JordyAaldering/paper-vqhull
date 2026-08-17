#!/bin/sh

#SBATCH --account=csmpi
#SBATCH --partition=csmpi_fpga_long
#SBATCH --nodelist=cn132
#SBATCH --mem=0
#SBATCH --cpus-per-task=32
#SBATCH --time=10:00:00
#SBATCH --output=threads_cn132.out

./scripts/init_energy.sh
./scripts/make.sh

iter=20
out="threads_cn132"

echo "bench,dataset,threads,size,runtime,rapl,rapl2" > res/"$out".csv

# Sweep thread-count only for 1e8 points
size=100000000
./scripts/vqhull.sh $size $iter $out
./scripts/pbbs.sh $size $iter $out
for threads in 2 4 8 16 32; do
  OMP_PROC_BIND=spread ./scripts/vqhull_par.sh $threads $size $iter $out numactl --interleave all
  ./scripts/pbbs_par.sh $threads $size $iter $out numactl --interleave all
done
