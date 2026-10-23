#include <algorithm>
#include <cstddef>
#include <iomanip>
#include <ios>
#include <locale>
#include <string>
#include <utility>

template <typename T1, typename T2> using t1 = std::time_put_byname<T1, T2>;

template <typename T1, typename T2>
std::time_put_byname<T1, T2> f1(const char *a0, std::size_t a1) {
  return std::time_put_byname<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::time_put_byname<T1, T2> f2(const std::string &a0, std::size_t a1) {
  return std::time_put_byname<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
T2 f3(const std::time_put_byname<T1, T2> &a0, T2 a1, std::ios_base &a2, T1 a3,
      const std::tm *a4, T1 const *a5, T1 const *a6) {
  return a0.put(a1, a2, a3, a4, a5, a6);
}

template <typename T1, typename T2>
T2 f4(const std::time_put_byname<T1, T2> &a0, T2 a1, std::ios_base &a2, T1 a3,
      const std::tm *a4, char a5, char a6) {
  return a0.put(a1, a2, a3, a4, a5, a6);
}
