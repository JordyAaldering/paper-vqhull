#!/bin/sh

#SBATCH --account=csmpi
#SBATCH --partition=csmpi_fpga_long
#SBATCH --mem=0
#SBATCH --cpus-per-task=32
#SBATCH --time=1:00:00
#SBATCH --output=stream_cn132.out

make clean
make stream_c
OMP_PROC_BIND=spread OMP_PLACES=cores OMP_NUM_THREADS=32 numactl -C 0-31 --interleave all ./stream_c
