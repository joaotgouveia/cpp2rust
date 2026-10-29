#include <algorithm>
#include <ios>
#include <iosfwd>
#include <utility>

template <typename T1> using t1 = std::numpunct<T1>;

template <typename T1> using t2 = typename std::numpunct<T1>::char_type;

template <typename T1> using t3 = typename std::numpunct<T1>::string_type;

template <typename T1> std::numpunct<T1> f1(std::size_t a0) {
  return std::numpunct<T1>(a0);
}

template <typename T1> T1 f2(const std::numpunct<T1> &a0) {
  return a0.decimal_point();
}

template <typename T1> T1 f3(const std::numpunct<T1> &a0) {
  return a0.thousands_sep();
}

template <typename T1> std::string f4(const std::numpunct<T1> &a0) {
  return a0.grouping();
}

template <typename T1> std::basic_string<T1> f5(const std::numpunct<T1> &a0) {
  return a0.truename();
}

template <typename T1> std::basic_string<T1> f6(const std::numpunct<T1> &a0) {
  return a0.falsename();
}

template <typename T1> const std::locale::id &f7() {
  return std::numpunct<T1>::id;
}
