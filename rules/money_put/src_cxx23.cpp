#include <algorithm>
#include <ios>
#include <locale>
#include <utility>

template <typename T1, typename T2> using t1 = std::money_put<T1, T2>;

template <typename T1, typename T2>
using t2 = typename std::money_put<T1, T2>::char_type;

template <typename T1, typename T2>
using t3 = typename std::money_put<T1, T2>::iter_type;

template <typename T1, typename T2>
using t4 = typename std::money_put<T1, T2>::string_type;

template <typename T1, typename T2> std::money_put<T1, T2> f1(std::size_t a0) {
  return std::money_put<T1, T2>(a0);
}

template <typename T1, typename T2>
T2 f2(const std::money_put<T1, T2> &a0, T2 a1, bool a2, std::ios_base &a3,
      T1 a4, long double a5) {
  return a0.put(std::move(a1), a2, a3, std::move(a4), a5);
}

template <typename T1, typename T2>
T2 f3(const std::money_put<T1, T2> &a0, T2 a1, bool a2, std::ios_base &a3,
      T1 a4, const std::basic_string<T1> &a5) {
  return a0.put(std::move(a1), a2, a3, std::move(a4), a5);
}

template <typename T1, typename T2> const std::locale::id &f4() {
  return std::money_put<T1, T2>::id;
}
