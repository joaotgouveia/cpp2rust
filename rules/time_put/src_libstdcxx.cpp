#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <ios>
#include <locale>
#include <utility>

template <typename T1, typename T2>
typename std::time_put<T1, T2>::iter_type
f4(const std::time_put<T1, T2> &a0,
   typename std::time_put<T1, T2>::iter_type a1, std::ios_base &a2,
   typename std::time_put<T1, T2>::char_type a3, const std::tm *a4,
   const T1 *a5, const T1 *a6) {
  return a0.put(a1, a2, a3, a4, a5, a6);
}
