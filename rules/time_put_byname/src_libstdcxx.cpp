#include <algorithm>
#include <ios>
#include <locale>
#include <string>
#include <utility>

template <typename T1, typename T2>
using t2 = typename std::time_put_byname<T1, T2>::char_type;

template <typename T1, typename T2>
using t3 = typename std::time_put_byname<T1, T2>::iter_type;

template <typename T1, typename T2>
typename std::time_put_byname<T1, T2>::iter_type
f4(const std::time_put_byname<T1, T2> &a0,
   typename std::time_put_byname<T1, T2>::iter_type a1, std::ios_base &a2,
   typename std::time_put_byname<T1, T2>::char_type a3, const std::tm *a4,
   T1 const *a5, T1 const *a6) {
  return a0.put(a1, a2, a3, a4, a5, a6);
}
