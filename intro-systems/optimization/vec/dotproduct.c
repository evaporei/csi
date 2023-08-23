#include "vec.h"


data_t dotproduct(vec_ptr u, vec_ptr v) {
   data_t sum = 0, u_val, v_val;

   for (long i = 0; i < vec_length(u); i++) { // we can assume both vectors are same length
        get_vec_element(u, i, &u_val);
        get_vec_element(v, i, &v_val);
        sum += u_val * v_val;
   }   
   return sum;
}
