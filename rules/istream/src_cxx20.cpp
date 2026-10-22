#include "../common/init.h"
#include "../common/traits.h"
#include <bitset>
#include <iosfwd>
#include <istream>
#include <utility>

template <typename T1, std::size_t T2>
std::basic_istream<char, T1> &f9(std::basic_istream<char, T1> &a0,
                                 unsigned char (&a1)[T2]) {
  return a0 >> a1;
}

template <typename T1, std::size_t T2>
std::basic_istream<char, T1> &f10(std::basic_istream<char, T1> &a0,
                                  signed char (&a1)[T2]) {
  return a0 >> a1;
}

template <typename T1, typename T2, std::size_t T3>
std::basic_istream<T1, T2> &f11(std::basic_istream<T1, T2> &a0, T1 (&a1)[T3]) {
  return a0 >> a1;
}
