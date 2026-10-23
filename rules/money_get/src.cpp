#include <algorithm>
#include <cstddef>
#include <iomanip>
#include <ios>
#include <locale>
#include <string>
#include <utility>

template <typename T1, typename T2> using t1 = std::money_get<T1, T2>;

template <typename T1, typename T2>
using t2 = typename std::money_get<T1, T2>::char_type;

template <typename T1, typename T2>
using t3 = typename std::money_get<T1, T2>::iter_type;

template <typename T1, typename T2>
using t4 = typename std::money_get<T1, T2>::string_type;

template <typename T1, typename T2> std::money_get<T1, T2> f1(std::size_t a0) {
  return std::money_get<T1, T2>(a0);
}

template <typename T1, typename T2>
T2 f2(const std::money_get<T1, T2> &a0, T2 a1, T2 a2, bool a3,
      std::ios_base &a4, std::ios_base::iostate &a5, long double &a6) {
  return a0.get(a1, a2, a3, a4, a5, a6);
}

template <typename T1, typename T2>
T2 f3(const std::money_get<T1, T2> &a0, T2 a1, T2 a2, bool a3,
      std::ios_base &a4, std::ios_base::iostate &a5,
      std::basic_string<T1> &a6) {
  return a0.get(a1, a2, a3, a4, a5, a6);
}

template <typename T1, typename T2> const std::locale::id &f4() {
  return std::money_get<T1, T2>::id;
}
