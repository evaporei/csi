/*
Naive code for multiplying two matrices together.

There must be a better way!
*/

#include <stdio.h>
#include <stdlib.h>

#define TILE_SIZE 32
#define min(a, b) (((a) < (b)) ? (a) : (b))

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

void fast_matrix_multiply(double **C, double **A, double **B, int a_rows,
                          int a_cols, int b_cols) {
  int ti, tj, tk,          // indexes of the tile
      i, j, k,             // indexes within a tile
      i_end, j_end, k_end; // end when matrix dim not a multiple of tile_size

  for (ti = 0; ti < a_rows; ti += TILE_SIZE) {
    i_end = min(ti + TILE_SIZE, a_rows);

    for (tj = 0; tj < b_cols; tj += TILE_SIZE) {
      j_end = min(tj + TILE_SIZE, b_cols);

      for (tk = 0; tk < a_cols; tk += TILE_SIZE) {
        k_end = min(tk + TILE_SIZE, a_cols);

        // Compute this tile
        for (i = ti; i < i_end; i++)
          for (j = tj; j < j_end; j++)
            for (k = tk; k < k_end; k++)
              C[i][j] += A[i][k] * B[k][j];
      }
    }
  }
}
