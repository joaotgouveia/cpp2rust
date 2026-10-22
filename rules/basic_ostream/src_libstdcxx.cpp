#include "../common/init.h"
#include "../common/traits.h"
#include <ios>
#include <ostream>
#include <string>
#include <utility>

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &
f55(std::basic_ostream<T1, T2> &a0,
    std::basic_ios<T1, T2> &(*a1)(std::basic_ios<T1, T2> &)) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f56(std::basic_ostream<T1, T2> &a0,
                                std::basic_streambuf<T1, T2> *a1) {
  return a0.operator<<(a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> *f57(const std::basic_ostream<T1, T2> &a0) {
  return a0.tie();
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> *f58(std::basic_ostream<T1, T2> &a0,
                                std::basic_ostream<T1, T2> *a1) {
  return a0.tie(a1);
}

template <typename T1, typename T2>
std::basic_streambuf<T1, T2> *f59(const std::basic_ostream<T1, T2> &a0) {
  return a0.rdbuf();
}

template <typename T1, typename T2>
std::basic_streambuf<T1, T2> *f60(std::basic_ostream<T1, T2> &a0,
                                  std::basic_streambuf<T1, T2> *a1) {
  return a0.rdbuf(a1);
}
