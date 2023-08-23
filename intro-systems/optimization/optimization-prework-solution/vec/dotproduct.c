#include "vec.h"

// Original: 9.5ns / element at -O0, 6.2ns/el at -O1 and -O2

/* data_t dotproduct(vec_ptr u, vec_ptr v) { */
/*    data_t sum = 0, u_val, v_val; */
/*  */
/*    for (long i = 0; i < vec_length(u); i++) { */
/*      get_vec_element(u, i, &u_val); */
/*      get_vec_element(v, i, &v_val); */
/*      sum += u_val * v_val; */
/*    } */
/*    return sum; */
/* } */

// Move vec_length: 8.5ns / element

/* data_t dotproduct(vec_ptr u, vec_ptr v) { */
/*   data_t sum = 0, u_val, v_val; */
/*   long n = vec_length(u); */
/*  */
/*   for (long i = 0; i < n; i++) { */
/*     get_vec_element(u, i, &u_val); */
/*     get_vec_element(v, i, &v_val); */
/*     sum += u_val * v_val; */
/*   } */
/*   return sum; */
/* } */

// Move get_vec_element: 2.4ns / element

/* data_t dotproduct(vec_ptr u, vec_ptr v) { */
/*   data_t sum = 0, *u_data = get_vec_start(u), *v_data = get_vec_start(v); */
/*   long n = vec_length(u); */
/*  */
/*   for (long i = 0; i < n; i++) { */
/*     sum += u_data[i] * v_data[i]; */
/*   } */
/*   return sum; */
/* } */

// Loop unrolling: 1.75ns / element

/* data_t dotproduct(vec_ptr u, vec_ptr v) { */
/*   data_t sum = 0, *u_data = get_vec_start(u), *v_data = get_vec_start(v); */
/*   long i, n = vec_length(u); */
/*  */
/*   for (i = 0; i < n - 3; i += 4) { */
/*     sum += u_data[i] * v_data[i]; */
/*     sum += u_data[i + 1] * v_data[i + 1]; */
/*     sum += u_data[i + 2] * v_data[i + 2]; */
/*     sum += u_data[i + 3] * v_data[i + 3]; */
/*   } */
/*  */
/*   for (; i < n; i++) { */
/*     sum += u_data[i] * v_data[i]; */
/*   } */
/*   return sum; */
/* } */

// Loop unrolling w/ multiple accumulators: 1.65 ns / element at -O0, 1.05ns/el
// at -O1

data_t dotproduct(vec_ptr u, vec_ptr v) {
  data_t sum1 = 0, sum2 = 0, sum3 = 0, sum4 = 0, *u_data = get_vec_start(u),
         *v_data = get_vec_start(v);
  long i, n = vec_length(u);

  for (i = 0; i < n - 3; i += 4) {
    sum1 += u_data[i] * v_data[i];
    sum2 += u_data[i + 1] * v_data[i + 1];
    sum3 += u_data[i + 2] * v_data[i + 2];
    sum4 += u_data[i + 3] * v_data[i + 3];
  }

  for (; i < n; i++) {
    sum1 += u_data[i] * v_data[i];
  }
  return sum1 + sum2 + sum3 + sum4;
}
