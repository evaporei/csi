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
    int a_cols, b_cols;
};

// AAAA I HATE THIS, HOW IS THIS NOT A PROBLEM??
int step_i = 0;

void *parallel_cb(void *ptr) {
    struct ParallelData *d = (struct ParallelData*) ptr;
    int i = step_i++;
    /* int i = d->i; */
    /* printf("after i: %d\n", i); */
    
    for (int j = 0; j < d->b_cols; j++) {
        for (int k = 0; k < d->a_cols; k++)
            d->c[i][j] += d->a[i][k] * d->b[k][j];
    }
}

void parallel_matrix_multiply(double **c, double **a, double **b, int a_rows,
                          int a_cols, int b_cols) {
    for (int i = 0; i < a_rows; i++) {
        for (int j = 0; j < b_cols; j++) {
            c[i][j] = 0;
        }
    }
    pthread_t threads[a_rows];
    int iret;

    struct ParallelData data;

    data.c = c;
    data.a = a;
    data.b = b;

    data.a_cols = a_cols;
    data.b_cols = b_cols;

    for (int i = 0; i < a_rows; i++) {
        /* data.i = i; */
        /* printf("before i: %d\n", i); */
        iret = pthread_create(&threads[i], NULL, parallel_cb, (void*) &data);
    }

    for (int i = 0; i < a_rows; i++) {
        pthread_join(threads[i], NULL);
    }
}
