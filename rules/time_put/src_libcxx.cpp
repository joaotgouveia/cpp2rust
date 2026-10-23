#include <cstddef>
#include <iomanip>
#include <ios>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2>
typename std::time_put<T1, T2>::iter_type
f5(const std::time_put<T1, T2> &a0,
   typename std::time_put<T1, T2>::iter_type a1, std::ios_base &a2,
   typename std::time_put<T1, T2>::char_type a3, const std::tm *a4,
   const typename std::time_put<T1, T2>::char_type *a5,
   const typename std::time_put<T1, T2>::char_type *a6) {
  return a0.put(a1, a2, a3, a4, a5, a6);
}

template <typename T1, typename T2> long f6(const std::time_put<T1, T2> &a0) {
  return a0.use_count();
}
