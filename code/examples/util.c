#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <sys/time.h>
#include <vqhull.h>
#include <string.h>
#include <omp.h>

#include "util.h"

void PrintPoints(size_t n, Points P)
{
    printf("pbbs_sequencePoint2d\n");
    for (size_t i = 0; i < n; i++) {
        printf("%.17e %.17e\n", P.x[i], P.y[i]);
    }
}

Points input(size_t *n /* out param */)
{
    size_t line_length;
    char *file_header = NULL;

    if (getline(&file_header, &line_length, stdin) == -1) {
        perror("Error in getting file header");
    }

    if (strcmp(file_header, "pbbs_sequencePoint2d\n") != 0) {
        printf("Error: not in pbbs_sequencePoint2d format\n");
        abort();
    }

    free(file_header);

    size_t size = 1000;
    Points P;
    P.x = (double *)malloc(size * sizeof(double));
    P.y = (double *)malloc(size * sizeof(double));

    size_t i = 0;
    while (scanf("%lf %lf", &P.x[i], &P.y[i]) == 2) {
        i++;
        if (i >= size) {
            size *= 2;
            P.x = (double *)realloc(P.x, size * sizeof(Point));
            P.y = (double *)realloc(P.y, size * sizeof(Point));
        }
    }

    *n = i;
    P.x = (double *)realloc(P.x, i * sizeof(Point));
    P.y = (double *)realloc(P.y, i * sizeof(Point));

    return P;
}

Points input_b(size_t *n /* out param */)
{
    constexpr size_t block = 2048; // 16 KiB
    size_t size = block / 2;
    Points P;
    P.x = (double *)malloc(size * sizeof(double));
    P.y = (double *)malloc(size * sizeof(double));

    double *buf = (double *)malloc(block * sizeof(double));
    size_t i = 0;
    size_t actual_read;
    do {
        actual_read = fread(buf, sizeof(double), block, stdin);
        if (i + actual_read / 2 > size) {
            size *= 2;
            P.x = (double *)realloc(P.x, size * sizeof(Point));
            P.y = (double *)realloc(P.y, size * sizeof(Point));
        }
        for (size_t j = 0; j < actual_read / 2; j++) {
            P.x[i + j] = buf[2 * j    ];
            P.y[i + j] = buf[2 * j + 1];
        }
        i += actual_read / 2;
    } while (actual_read == block);

    *n = i;
    P.x = (double *)realloc(P.x, i * sizeof(Point));
    P.y = (double *)realloc(P.y, i * sizeof(Point));

    free(buf);

    return P;
}

#if 0
double wtime(void)
{
    struct timespec ts;
    clock_gettime(CLOCK_REALTIME, &ts);
    return (double)(ts.tv_nsec / 1e9);
}
#else
double wtime(void)
{
    struct timeval tv;
    gettimeofday(&tv, NULL);
    return 1e-6 * (double)tv.tv_usec  + (double)tv.tv_sec;
}
#endif

void warmup(size_t n, Points P, int runs)
{
    for (int t = 0; t < runs; t++) {
        #pragma omp parallel for
        for (size_t i = 0; i < n; i++) {
            P.x[i] *= 2;
            P.y[i] *= 2;
        }
        #pragma omp parallel for
        for (size_t i = 0; i < n; i++) {
            P.x[i] /= 2;
            P.y[i] /= 2;
        }
    }
}
