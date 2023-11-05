/*
  A very simple test and benchmark suite
*/
#include <stdlib.h>
#include <stdio.h>
#include <stdbool.h>
#include <sys/time.h>
#include "matrix-multiply.h"


// Allocate space for an m x n matrix. Caller frees.
double **matrix_alloc(int m, int n) {
  double **matrix = malloc(m * sizeof(double));

  for (int i = 0; i < m; i++)
    matrix[i] = calloc(n, sizeof(double));

  return matrix;
}

// Free the entirety of an m row matrix
void matrix_free(double **matrix, int m) {
  for (int i = 0; i < m; i++)
    free(matrix[i]);
  free(matrix);
}

// Fill an m x n matrix with random values
void matrix_fill_random(double **matrix, int m, int n) {
  for (int i = 0; i < m; i++)
    for (int j = 0; j < n; j++)
      matrix[i][j] = (double)rand() / (double)RAND_MAX;
}

// Verify that two m x n matrices contain the same values
bool matrix_equal(double **A, double **B, int m, int n) {
  for (int i = 0; i < m; i++)
    for (int j = 0; j < n; j++)
      if (A[i][j] != B[i][j])
        return false;
  return true;
}

// To ensure fair cache pre-population, write junk to cache to "flush" it
void flush_cache() {
  int size = 4 * 1024 * 1024; // 4MB to clear out L3
  char *b = malloc(size);
  for (int i = 0; i < size; i++)
    b[i] = i;
  free(b);
}

int main(int argc, char *argv[]) {
  if (argc != 2) {
    printf("Usage: ./benchmark [n]\n");
    exit(1);
  }

  int n = atoi(argv[1]);
  struct timeval start, stop;
  double naive_time, parallel_time;

  // alloc input and output matrices
  double **A = matrix_alloc(n, n);
  double **B = matrix_alloc(n, n);
  double **C_naive = matrix_alloc(n, n);
  double **C_parallel = matrix_alloc(n, n);

  // input matrices should have random values
  matrix_fill_random(A, n, n);
  matrix_fill_random(B, n, n);

  // compute the product naively
  flush_cache();
  gettimeofday(&start, NULL);
  matrix_multiply(C_naive, A, B, n, n, n);
  gettimeofday(&stop, NULL);
  naive_time = (stop.tv_sec - start.tv_sec) * 1000000 + stop.tv_usec - start.tv_usec;

  // compute the product using parallelism
  flush_cache();
  gettimeofday(&start, NULL);
  parallel_matrix_multiply(C_parallel, A, B, n, n, n);
  gettimeofday(&stop, NULL);
  parallel_time = (stop.tv_sec - start.tv_sec) * 1000000 + stop.tv_usec - start.tv_usec;

  printf("Naive: %.3fs\nParallel: %.3fs\n", naive_time / 1000000, parallel_time / 1000000);
  printf("%0.2fx speedup between naive and parallel\n", naive_time / parallel_time);

  // verify that all outputs are the same
  if (!matrix_equal(C_naive, C_parallel, n, n))
    printf("\nNaive result did not match parallel result!\n");

  // free everything
  matrix_free(A, n);
  matrix_free(B, n);
  matrix_free(C_naive, n);
  matrix_free(C_parallel, n);

  return 0;
}
