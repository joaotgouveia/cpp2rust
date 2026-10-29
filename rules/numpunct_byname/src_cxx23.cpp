#include <algorithm>
#include <ios>
#include <iosfwd>
#include <utility>

template <typename T1> using t1 = std::numpunct_byname<T1>;

template <typename T1> using t2 = typename std::numpunct_byname<T1>::char_type;

template <typename T1>
using t3 = typename std::numpunct_byname<T1>::string_type;

template <typename T1>
std::numpunct_byname<T1> f1(const char *a0, std::size_t a1) {
  return std::numpunct_byname<T1>(a0, a1);
}

template <typename T1>
std::numpunct_byname<T1> f2(const std::string &a0, std::size_t a1) {
  return std::numpunct_byname<T1>(a0, a1);
}

template <typename T1> T1 f3(const std::numpunct_byname<T1> &a0) {
  return a0.decimal_point();
}

template <typename T1> T1 f4(const std::numpunct_byname<T1> &a0) {
  return a0.thousands_sep();
}

template <typename T1> std::string f5(const std::numpunct_byname<T1> &a0) {
  return a0.grouping();
}

template <typename T1>
std::basic_string<T1> f6(const std::numpunct_byname<T1> &a0) {
  return a0.truename();
}

template <typename T1>
std::basic_string<T1> f7(const std::numpunct_byname<T1> &a0) {
  return a0.falsename();
}
