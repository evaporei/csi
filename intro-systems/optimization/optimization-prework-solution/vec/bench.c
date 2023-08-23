#include <stdio.h>
#include <time.h>

#include "vec.h"

int main(void) {
  long n = 100000000;
  vec_ptr u = new_vec(n);
  vec_ptr v = new_vec(n);

  for (long i = 0; i < n; i++) {
    set_vec_element(u, i, i + 1);
    set_vec_element(v, i, i + 1);
  }

  /* long expected = (2 * n * n * n + 3 * n * n + n) / 6; */

  clock_t start = clock();
  long actual = dotproduct(u, v);
  clock_t end = clock();

  /* TEST_ASSERT_EQUAL(expected, actual); */
  double time_elapsed = (end - start) / (double)CLOCKS_PER_SEC;
  printf(
      "%0.2fs to take product of length %ld vectors (%0.2f ns per element)\n",
      time_elapsed, n, time_elapsed * 1e9 / n);

  free_vec(u);
  free_vec(v);
}
