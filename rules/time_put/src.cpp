#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <cstddef>
#include <iomanip>
#include <ios>
#include <locale>
#include <utility>

template <typename T1, typename T2> using t1 = std::time_put<T1, T2>;

template <typename T1, typename T2>
using t2 = typename std::time_put<T1, T2>::char_type;

template <typename T1, typename T2>
using t3 = typename std::time_put<T1, T2>::iter_type;

template <typename T1, typename T2> std::time_put<T1, T2> f1(std::size_t a0) {
  return std::time_put<T1, T2>(a0);
}

template <typename T1, typename T2>
typename std::time_put<T1, T2>::iter_type
f2(const std::time_put<T1, T2> &a0,
   typename std::time_put<T1, T2>::iter_type a1, std::ios_base &a2,
   typename std::time_put<T1, T2>::char_type a3, const std::tm *a4, char a5,
   char a6) {
  return a0.put(a1, a2, a3, a4, a5, a6);
}

template <typename T1, typename T2> const std::locale::id &f3() {
  return std::time_put<T1, T2>::id;
}
