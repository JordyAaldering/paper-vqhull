#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <omp.h>

#include "util.h"

int main(int argc, char **argv)
{
    if (argc != 2) {
        printf("Usage: %s ITER\n", argv[0]);
        return EXIT_FAILURE;
    }

    int iter = atoi(argv[1]);
    int max_threads;

    #pragma omp parallel master
    max_threads = omp_get_num_threads();

    int test = 0;

    for (int nthreads = 2; nthreads <= max_threads; nthreads *= 2) {
        double start = wtime();
        for (int i = 0; i < iter; i++) {
            #pragma omp parallel num_threads(nthreads)
            {
                if (omp_get_thread_num() == 0) test++;
            }
        }
        double stop = wtime();

        printf("Creating %d threads, %e mus total.\n", 
                nthreads,
                1e6 * nthreads / (stop - start) / iter);
    }

    return EXIT_SUCCESS;
}
