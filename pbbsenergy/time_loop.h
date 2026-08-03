#include "../parlay/internal/get_time.h"

#include <energy_bench.h>

template<class F, class G, class H>
void time_loop(int rounds, double delay, F initf, G runf, H endf) {
  parlay::internal::timer t;
  // run for delay seconds to "warm things up"
  // will skip if delay is zero
  while (t.total_time() < delay) {
    initf(); runf(); endf();
  }
  for (int i=0; i < rounds; i++) {
    const uintptr_t idle_duration_seconds = 300;
    struct BenchStart *bench = energy_bench_init(idle_duration_seconds);
    struct BenchResult *energy;

    initf();

    energy_bench_start(bench);

    runf();

    energy = energy_bench_stop(bench);

    printf("%f", energy->runtime);
    for (uintptr_t i = 0; i < energy->len; i++) {
      printf(",%f", energy->values[i]);
    }
    printf("\n");

    energy_bench_free(bench);
    energy_bench_res_free(energy);

    endf();
  }
}
