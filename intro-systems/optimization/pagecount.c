#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>

/* #define TEST_LOOPS 10 */
#define TEST_LOOPS 10000000

// NOTE: we know that memory_size and page_size are both powers of 2, so one
// divides the other
//

uint64_t pagecount(uint64_t memory_size, uint64_t page_size) {
  /* return memory_size / page_size; */
  return memory_size >> page_size;

  /* uint64_t a = __builtin_ffsl(memory_size); */
  /* uint64_t b = __builtin_ffsl(page_size); */
  /* printf("memory_size: %llu, page_size: %llu, a: %llu, b: %llu\n", memory_size, page_size, a, b); */
  /* printf("memory_size / page_size: %llu, 1LLU << (a - b): %llu\n", memory_size / page_size, 1LLU << (a - b)); */
  /* return 1LLU << (a - b); */
}

int main (int argc, char** argv) {
  clock_t baseline_start, baseline_end, test_start, test_end;
  uint64_t memory_size, page_size;
  double clocks_elapsed, time_elapsed;
  int i, ignore = 0;

  uint64_t msizes[] = {1L << 32, 1L << 40, 1L << 52};
  uint64_t psizes[] = {1L << 12, 1L << 16, 1L << 32};

  baseline_start = clock();
  for (i = 0; i < TEST_LOOPS; i++) {
    memory_size = msizes[i % 3];
    page_size = psizes[i % 3];
    ignore += 1 + memory_size +
              page_size; // so that this loop isn't just optimized away
  }
  baseline_end = clock();

  test_start = clock();
  for (i = 0; i < TEST_LOOPS; i++) {
    memory_size = msizes[i % 3];
    page_size = psizes[i % 3];
    ignore += pagecount(memory_size, page_size) + memory_size + page_size;
  }
  test_end = clock();

  clocks_elapsed = test_end - test_start - (baseline_end - baseline_start);
  time_elapsed = clocks_elapsed / CLOCKS_PER_SEC;

  printf("%.2fs to run %d tests (%.2fns per test)\n", time_elapsed, TEST_LOOPS,
         time_elapsed * 1e9 / TEST_LOOPS);
  return ignore;
}

