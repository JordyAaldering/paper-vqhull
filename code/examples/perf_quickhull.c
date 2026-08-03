#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <vqhull.h>
#include <omp.h>
#include <unistd.h>
#include <signal.h>

#include "util.h"

int main(int argc, char **argv)
{
    if (argc > 2) {
        printf("Usage: Provide points on stdin in PBBS format.\n"
               "b accepts binary input\n");
        return EXIT_FAILURE;
    }

    bool binary = false;

    for (int i = 1; i < argc; i++) {
        if (strcmp(argv[i], "b") == 0) {
            binary = true;
        } else {
            printf("%s is not a valid option\n", argv[0]);
            return EXIT_FAILURE;
        }
    }

    size_t n;
    Points P = (binary) ? input_b(&n) : input(&n);

    int pid  = getpid();
    int cpid = fork();

    if (cpid == 0) {
        // child process .  Run your perf stat
        char *buf;
        asprintf(&buf, "perf stat -a -M DRAM_BW_Use -e unc_m_cas_count_rd,unc_m_cas_count_wr,unc_m_vc0_requests_rd,unc_m_vc0_requests_wr,offcore_requests.demand_data_rd,offcore_requests.data_rd,unc_m_prefetch_rd");
        execl("/bin/sh", "sh", "-c", buf, NULL);
    } else {
        setpgid(cpid, 0);

        size_t count = VecQuickhullP(n, P.x, P.y);

        kill(-cpid, SIGINT);
    }

    free(P.x);
    free(P.y);

    return EXIT_SUCCESS;
}
