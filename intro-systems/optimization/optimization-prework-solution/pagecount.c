/*
 * Optimization notes:
 *
 * - Tested on Intel i5-7360U (Kaby Lake) CPU @ 2.30GHz
 * - Pre-optimized version takes 6-8ns per test, depending on optimization leveA
 * - `div` of a 64 bit register is expected to be the most expensive op, at 36
 * uops total, 35-88 uops latency, 21-83 uops reciprocal throuphput according to
 * Agner Fog
 * - By moving to a bit shift, we should expect to be able to eliminate MOST of
 * the execution time, since it's largely attributable to `div`
 * - Indeed, we see 0.25-2.5ns per test depending on optimization level
 * - This is not an optimization the compiler can perform, as it couldn't
 * possible know that page_size would always be a power of 2
 */

#include <stdint.h>
#include <stdio.h>
#include <time.h>

#define TEST_LOOPS 10000000

uint64_t pagecount(uint64_t memory_size, uint64_t page_scale) {
  return memory_size >> page_scale;
}

int main (int argc, char** argv) {
  clock_t baseline_start, baseline_end, test_start, test_end;
  uint64_t memory_size, page_scale, ret;
  double clocks_elapsed, time_elapsed;
  int i, ignore = 0;

  uint64_t msizes[] = {1L << 32, 1L << 40, 1L << 52};
  uint64_t pscales[] = {12, 16, 32};

  baseline_start = clock();
  for (i = 0; i < TEST_LOOPS; i++) {
    memory_size = msizes[i % 3];
    page_scale = pscales[i % 3];
    ignore += 1 + memory_size +
              page_scale; // so that this loop isn't just optimized away
  }
  baseline_end = clock();

  ret = 1;
  test_start = clock();
  for (i = 0; i < TEST_LOOPS; i++) {
    memory_size = msizes[i % 3];
    page_scale = pscales[i % 3];
    ret = pagecount(memory_size, page_scale);
    ignore += ret + memory_size + page_scale;
  }
  test_end = clock();

  clocks_elapsed = test_end - test_start - (baseline_end - baseline_start);
  time_elapsed = clocks_elapsed / CLOCKS_PER_SEC;

  printf("%.2fs to run %d tests (%.2fns per test)\n", time_elapsed, TEST_LOOPS,
         time_elapsed * 1e9 / TEST_LOOPS);
  return ignore;
}

