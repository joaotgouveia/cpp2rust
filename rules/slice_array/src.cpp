#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <utility>
#include <valarray>

template <typename T1> using t1 = std::slice_array<T1>;

template <typename T1> using t2 = typename std::slice_array<T1>::value_type;

template <typename T1> std::slice_array<T1> f1(const std::slice_array<T1> &a0) {
  return std::slice_array<T1>(a0);
}
