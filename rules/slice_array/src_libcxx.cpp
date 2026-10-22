#include "../common/init.h"
#include "../common/traits.h"
#include <utility>
#include <valarray>

template <typename T1>
const std::slice_array<T1> &f25(const std::slice_array<T1> &a0,
                                const std::slice_array<T1> &a1) {
  return a0.operator=(a1);
}

template <typename T1>
void f26(const std::slice_array<T1> &a0,
         const typename std::slice_array<T1>::value_type &a1) {
  return a0.operator=(a1);
}

template <typename T1>
void f27(const std::slice_array<T1> &a0,
         const std::valarray<typename std::slice_array<T1>::value_type> &a1) {
  return a0.operator=(a1);
}
