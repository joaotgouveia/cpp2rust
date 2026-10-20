#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <utility>

template <typename T1> using t1 = std::const_iterator<T1>;

template <typename T1> using t2 = std::const_sentinel<T1>;

template <typename T1> using t3 = std::iter_const_reference_t<T1>;

template <typename T1> std::const_iterator<T1> f38(T1 a0) {
  return std::make_const_iterator<T1>(a0);
}

template <typename T1> std::const_sentinel<T1> f39(T1 a0) {
  return std::make_const_sentinel<T1>(a0);
}
