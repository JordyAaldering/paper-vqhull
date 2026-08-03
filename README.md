## Running the benchmarks

Be aware that the very first benchmark run will take much longer, as in the first iteration we first automatically measure
idle power draw for five minutes. This value is cached, and is recomputed after 6 days, or if the cache file is removed
(default location is `~/.local/share/idle-*.json`)

1) Setup _Highway_.
    * Clone the _Highway_ repository: `git clone https://github.com/google/highway.git`.
    * Build the project. You probably want to do this as a SLURM job.
      We use the following script:
      ```sh
      #!/bin/sh
      #SBATCH --account=csmpi
      #SBATCH --partition=csmpi_long
      #SBATCH --mem=0
      #SBATCH --cpus-per-task=16
      #SBATCH --time=1:00:00
      #SBATCH --output=build.out

      rm -rf build || exit
      mkdir build && cd build
      cmake -DCMAKE_INSTALL_PREFIX="${HOME}/.local" -DHWY_ENABLE_TESTS=FALSE -DHWY_ENABLE_EXAMPLES=FALSE -DCONTRIB_ENABLE_CONTRIB=FALSE ..
      make -j
      ```
2) We use the _EnergyBench_ tool to measure energy consumption across RAPL packages and automatically exclude idle power draw.
    * Clone the _EnergyBench_ repository: `git clone https://gitlab.com/software-energy-lab/energy-benchmarking.git`.
    * Change to the C FFI directory: `cd energy_benchmarking/ffi`.
    * Run `make install`, which builds the Rust project (with _cargo_) and copies the shared library and header file to `~/.local`.
3) Ensure the _pbbsbench_ submodule is initialized: `git submodule update --init --recursive`.
4) We extend _pbbsbench_ with energy measurements.
   For this, run `./initialize_energy.sh`, which copies the relevant files into the _pbbsbench_ submodule.
5) Generate input data with _N_ points: `./gen_input.sh <N>` for _N_ is 1e5, 1e6, 1e7, 1e8, and 1e9.
6) The `scripts` foldeer contains the scripts to run the benchmarks.
    * If a device is added, copy one of these folders and adjust the (SLURM) configuration accordingly for the hardware.

For the paper, we run the benchmarks on the Radboud C&CZ cluster.
This additionally requires access to the `rapl` group for energy measurements, and to the `itt` group for access to `/vol/itt` to store the large datasets.
