#include <algorithm>
#include <ios>
#include <iosfwd>
#include <locale>
#include <utility>

template <typename T1, typename T2> using t1 = std::time_get_byname<T1, T2>;

template <typename T1, typename T2>
using t2 = typename std::time_get_byname<T1, T2>::char_type;

template <typename T1, typename T2>
using t3 = typename std::time_get_byname<T1, T2>::iter_type;

template <typename T1, typename T2>
std::time_get_byname<T1, T2> f1(const char *a0, std::size_t a1) {
  return std::time_get_byname<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::time_get_byname<T1, T2> f2(const std::string &a0, std::size_t a1) {
  return std::time_get_byname<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
typename std::time_get_byname<T1, T2>::dateorder
f3(const std::time_get_byname<T1, T2> &a0) {
  return a0.date_order();
}

template <typename T1, typename T2>
T2 f4(const std::time_get_byname<T1, T2> &a0, T2 a1, T2 a2, std::ios_base &a3,
      std::ios_base::iostate &a4, std::tm *a5) {
  return a0.get_time(std::move(a1), std::move(a2), a3, a4, a5);
}

template <typename T1, typename T2>
T2 f5(const std::time_get_byname<T1, T2> &a0, T2 a1, T2 a2, std::ios_base &a3,
      std::ios_base::iostate &a4, std::tm *a5) {
  return a0.get_date(std::move(a1), std::move(a2), a3, a4, a5);
}

template <typename T1, typename T2>
T2 f6(const std::time_get_byname<T1, T2> &a0, T2 a1, T2 a2, std::ios_base &a3,
      std::ios_base::iostate &a4, std::tm *a5) {
  return a0.get_weekday(std::move(a1), std::move(a2), a3, a4, a5);
}

template <typename T1, typename T2>
T2 f7(const std::time_get_byname<T1, T2> &a0, T2 a1, T2 a2, std::ios_base &a3,
      std::ios_base::iostate &a4, std::tm *a5) {
  return a0.get_monthname(std::move(a1), std::move(a2), a3, a4, a5);
}

template <typename T1, typename T2>
T2 f8(const std::time_get_byname<T1, T2> &a0, T2 a1, T2 a2, std::ios_base &a3,
      std::ios_base::iostate &a4, std::tm *a5) {
  return a0.get_year(std::move(a1), std::move(a2), a3, a4, a5);
}

template <typename T1, typename T2>
T2 f9(const std::time_get_byname<T1, T2> &a0, T2 a1, T2 a2, std::ios_base &a3,
      std::ios_base::iostate &a4, std::tm *a5, char a6, char a7) {
  return a0.get(std::move(a1), std::move(a2), a3, a4, a5, a6, a7);
}

template <typename T1, typename T2>
T2 f10(const std::time_get_byname<T1, T2> &a0, T2 a1, T2 a2, std::ios_base &a3,
       std::ios_base::iostate &a4, std::tm *a5, T1 const *a6, T1 const *a7) {
  return a0.get(std::move(a1), std::move(a2), a3, a4, a5, a6, a7);
}

template <typename T1, typename T2> const std::locale::id &f11() {
  return std::time_get_byname<T1, T2>::id;
}
