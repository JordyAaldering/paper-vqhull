#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <vqhull.h>
#include <omp.h>

#ifdef ENERGY
#include <energy_bench.h>
#endif

#include "util.h"

int main(int argc, char **argv)
{
    if (argc == 1 || argc > 5) {
        printf("Usage: Provide points on stdin in PBBS format.\n"
               "s will print summary of results to stderr\n"
               "b accepts binary input\n"
               "m will print runtime in seconds to stdout\n"
               "p will print points to stdout");
        return EXIT_FAILURE;
    }

    bool summary = false;
    bool print = false;
    bool binary = false;
    bool measure = false;

    for (int i = 1; i < argc; i++) {
        if (strcmp(argv[i], "s") == 0) {
            summary = true;
        } else if (strcmp(argv[i], "p") == 0) {
            print = true;
        } else if (strcmp(argv[i], "b") == 0) {
            binary = true;
        } else if (strcmp(argv[i], "m") == 0) {
            measure = true;
        } else {
            printf("%s is not a valid option\n", argv[0]);
            return EXIT_FAILURE;
        }
    }

    #ifdef ENERGY
    const uintptr_t idle_duration_seconds = 300;
    struct BenchStart *bench = energy_bench_init(idle_duration_seconds);
    struct BenchResult *energy;
    #endif

    size_t n;
    Points P = (binary) ? input_b(&n) : input(&n);

    #ifdef ENERGY
    energy_bench_start(bench);
    #else
    double start = wtime();
    #endif

    size_t count = VecQuickhullP(n, P.x, P.y);

    #ifdef ENERGY
    energy = energy_bench_stop(bench);
    double duration = (double)energy->runtime;
    #else
    double duration = wtime() - start;
    #endif

    if (summary) {
        fprintf(stderr, "We found %zu elements on the hull\n", count);
        fprintf(stderr, "Runtime in ms: %lf\n", duration * 1e3);
        fprintf(stderr, "Bandwidth in MB/s: %lf\n",
                    n * 2 * sizeof(double) / duration / 1e6);
    }

    if (measure) {
        printf("%lf", duration);
        #ifdef ENERGY
        for (uintptr_t i = 0; i < energy->len; i++) {
            printf(",%f", energy->values[i]);
        }
        #endif
        printf("\n");
    }

    if (print) {
        PrintPoints(count, P);
    }

    free(P.x);
    free(P.y);
    #ifdef ENERGY
    energy_bench_free(bench);
    energy_bench_res_free(energy);
    #endif

    return EXIT_SUCCESS;
}
