#include "../common/init.h"
#include "../common/traits.h"
#include <bitset>
#include <iosfwd>
#include <istream>
#include <string>
#include <utility>

template <typename T1>
std::basic_istream<char, T1> &f1(std::basic_istream<char, T1> &a0,
                                 unsigned char &a1) {
  return a0 >> a1;
}

template <typename T1>
std::basic_istream<char, T1> &f2(std::basic_istream<char, T1> &a0,
                                 signed char &a1) {
  return a0 >> a1;
}

template <typename T1, typename T2> T1 &&f3(T1 &&a0, T2 &&a1) {
  return std::forward<T1>(a0) >> std::forward<T2>(a1);
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &f4(std::basic_istream<T1, T2> &a0, T1 &a1) {
  return a0 >> a1;
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &f5(std::basic_istream<T1, T2> &a0) {
  return std::ws<T1, T2>(a0);
}
