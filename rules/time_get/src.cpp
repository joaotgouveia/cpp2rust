#include <algorithm>
#include <cstddef>
#include <iomanip>
#include <ios>
#include <locale>
#include <utility>

template <typename T1, typename T2> using t1 = std::time_get<T1, T2>;

template <typename T1, typename T2>
using t2 = typename std::time_get<T1, T2>::char_type;

template <typename T1, typename T2>
using t3 = typename std::time_get<T1, T2>::iter_type;

template <typename T1, typename T2> std::time_get<T1, T2> f1(std::size_t a0) {
  return std::time_get<T1, T2>(a0);
}

template <typename T1, typename T2>
typename std::time_get<T1, T2>::dateorder f2(const std::time_get<T1, T2> &a0) {
  return a0.date_order();
}

template <typename T1, typename T2>
T2 f3(const std::time_get<T1, T2> &a0, T2 a1, T2 a2, std::ios_base &a3,
      std::ios_base::iostate &a4, std::tm *a5) {
  return a0.get_time(a1, a2, a3, a4, a5);
}

template <typename T1, typename T2>
T2 f4(const std::time_get<T1, T2> &a0, T2 a1, T2 a2, std::ios_base &a3,
      std::ios_base::iostate &a4, std::tm *a5) {
  return a0.get_date(a1, a2, a3, a4, a5);
}

template <typename T1, typename T2>
T2 f5(const std::time_get<T1, T2> &a0, T2 a1, T2 a2, std::ios_base &a3,
      std::ios_base::iostate &a4, std::tm *a5) {
  return a0.get_weekday(a1, a2, a3, a4, a5);
}

template <typename T1, typename T2>
T2 f6(const std::time_get<T1, T2> &a0, T2 a1, T2 a2, std::ios_base &a3,
      std::ios_base::iostate &a4, std::tm *a5) {
  return a0.get_monthname(a1, a2, a3, a4, a5);
}

template <typename T1, typename T2>
T2 f7(const std::time_get<T1, T2> &a0, T2 a1, T2 a2, std::ios_base &a3,
      std::ios_base::iostate &a4, std::tm *a5) {
  return a0.get_year(a1, a2, a3, a4, a5);
}

template <typename T1, typename T2>
T2 f8(const std::time_get<T1, T2> &a0, T2 a1, T2 a2, std::ios_base &a3,
      std::ios_base::iostate &a4, std::tm *a5, char a6, char a7) {
  return a0.get(a1, a2, a3, a4, a5, a6, a7);
}

template <typename T1, typename T2>
T2 f9(const std::time_get<T1, T2> &a0, T2 a1, T2 a2, std::ios_base &a3,
      std::ios_base::iostate &a4, std::tm *a5, const T1 *a6, const T1 *a7) {
  return a0.get(a1, a2, a3, a4, a5, a6, a7);
}

template <typename T1, typename T2> const std::locale::id &f10() {
  return std::time_get<T1, T2>::id;
}
