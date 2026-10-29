#include <algorithm>
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
T2 f2(const std::time_put<T1, T2> &a0, T2 a1, std::ios_base &a2, T1 a3,
      const std::tm *a4, const T1 *a5, const T1 *a6) {
  return a0.put(std::move(a1), a2, std::move(a3), a4, a5, a6);
}

template <typename T1, typename T2>
T2 f3(const std::time_put<T1, T2> &a0, T2 a1, std::ios_base &a2, T1 a3,
      const std::tm *a4, char a5, char a6) {
  return a0.put(std::move(a1), a2, std::move(a3), a4, a5, a6);
}

template <typename T1, typename T2> const std::locale::id &f4() {
  return std::time_put<T1, T2>::id;
}
