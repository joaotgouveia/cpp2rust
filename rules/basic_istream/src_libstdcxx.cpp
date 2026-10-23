#include <istream>
#include <string>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2>
std::basic_istream<T1, T2> &
f64(std::basic_istream<T1, T2> &a0,
    std::basic_ios<T1, T2> &(*a1)(std::basic_ios<T1, T2> &)) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &f65(std::basic_istream<T1, T2> &a0,
                                std::basic_streambuf<T1, T2> *a1) {
  return a0.operator>>(a1);
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &
f66(std::basic_istream<T1, T2> &a0, std::basic_streambuf<T1, T2> &a1,
    typename std::basic_istream<T1, T2>::char_type a2) {
  return a0.get(a1, a2);
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &f67(std::basic_istream<T1, T2> &a0,
                                std::basic_streambuf<T1, T2> &a1) {
  return a0.get(a1);
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &f68(std::basic_istream<T1, T2> &a0,
                                std::streamsize a1) {
  return a0.ignore(a1);
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &f69(std::basic_istream<T1, T2> &a0) {
  return a0.ignore();
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> *f70(const std::basic_istream<T1, T2> &a0) {
  return a0.tie();
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> *f71(std::basic_istream<T1, T2> &a0,
                                std::basic_ostream<T1, T2> *a1) {
  return a0.tie(a1);
}

template <typename T1, typename T2>
std::basic_streambuf<T1, T2> *f72(const std::basic_istream<T1, T2> &a0) {
  return a0.rdbuf();
}

template <typename T1, typename T2>
std::basic_streambuf<T1, T2> *f73(std::basic_istream<T1, T2> &a0,
                                  std::basic_streambuf<T1, T2> *a1) {
  return a0.rdbuf(a1);
}
