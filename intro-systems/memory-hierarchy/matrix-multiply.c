/*
Naive code for multiplying two matrices together.

There must be a better way!
*/

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

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

void matrix_init(double **matrix, int m, int n) {
  for (int i = 0; i < m; i++) {
    memset(matrix[i], 0, n * sizeof(double));
  }
}

void fast_matrix_multiply(double **C, double **A, double **B, int a_rows,
                          int a_cols, int b_cols) {
  matrix_init(C, a_rows, b_cols);
  for (int i = 0; i < a_rows; i++) {
    for (int k = 0; k < a_cols; k++)
      for (int j = 0; j < b_cols; j++) {
        C[i][j] += A[i][k] * B[k][j];
    }
  }
}
