#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>

/*
  A naive implementation of matrix multiplication.

  DO NOT MODIFY THIS FUNCTION, the tests assume it works correctly, which it
  currently does
*/
void matrix_multiply(double **C, double **A, double **B, int a_rows, int a_cols,
                     int b_cols) {
  for (int i = 0; i < a_rows; i++) {
    for (int j = 0; j < b_cols; j++) {
      C[i][j] = 0;
      for (int k = 0; k < a_cols; k++)
        C[i][j] += A[i][k] * B[k][j];
    }
  }
}

struct ParallelData {
    double **c, **a, **b;
    int a_rows, a_cols, b_cols;
};

void *parallel_cb(void *ptr) {
    struct ParallelData *d = (struct ParallelData*) ptr;
    
    for (int i = 0; i < d->a_rows; i++) {
        for (int j = 0; j < d->b_cols; j++) {
            d->c[i][j] = 0;
            for (int k = 0; k < d->a_cols; k++)
                d->c[i][j] += d->a[i][k] * d->b[k][j];
        }
    }
}

void parallel_matrix_multiply(double **c, double **a, double **b, int a_rows,
                          int a_cols, int b_cols) {
    pthread_t thread1;
    int iret1;

    struct ParallelData data;

    data.c = c;
    data.a = a;
    data.b = b;

    data.a_rows = a_rows;
    data.a_cols = a_cols;
    data.b_cols = b_cols;

    iret1 = pthread_create(&thread1, NULL, parallel_cb, (void*) &data);
    pthread_join(thread1, NULL);
}
