#include <algorithm>
#include <cstddef>
#include <ios>
#include <regex>
#include <string>
#include <utility>

template <typename T1> using t1 = std::regex_traits<T1>;

template <typename T1> using t2 = typename std::regex_traits<T1>::char_type;

template <typename T1> using t3 = typename std::regex_traits<T1>::string_type;

template <typename T1> using t4 = typename std::regex_traits<T1>::locale_type;

template <typename T1> std::regex_traits<T1> f1() {
  return std::regex_traits<T1>();
}

template <typename T1> std::size_t f2(const T1 *a0) {
  return std::regex_traits<T1>::length(a0);
}

template <typename T1> T1 f3(const std::regex_traits<T1> &a0, T1 a1) {
  return a0.translate(a1);
}

template <typename T1> T1 f4(const std::regex_traits<T1> &a0, T1 a1) {
  return a0.translate_nocase(a1);
}

template <typename T1> int f5(const std::regex_traits<T1> &a0, T1 a1, int a2) {
  return a0.value(a1, a2);
}

template <typename T1>
std::locale f6(std::regex_traits<T1> &a0, std::locale a1) {
  return a0.imbue(std::move(a1));
}

template <typename T1> std::locale f7(const std::regex_traits<T1> &a0) {
  return a0.getloc();
}

template <typename T1, typename T2>
std::basic_string<T1> f8(const std::regex_traits<T1> &a0, T2 a1, T2 a2) {
  return a0.template transform<T2>(a1, a2);
}

template <typename T1, typename T2>
std::basic_string<T1> f9(const std::regex_traits<T1> &a0, T2 a1, T2 a2) {
  return a0.template transform_primary<T2>(a1, a2);
}

template <typename T1, typename T2>
std::basic_string<T1> f10(const std::regex_traits<T1> &a0, T2 a1, T2 a2) {
  return a0.template lookup_collatename<T2>(a1, a2);
}
