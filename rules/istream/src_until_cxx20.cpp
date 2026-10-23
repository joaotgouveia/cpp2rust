#include <bitset>
#include <iosfwd>
#include <istream>
#include <string>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1>
std::basic_istream<char, T1> &f5(std::basic_istream<char, T1> &a0,
                                 unsigned char *a1) {
  return a0 >> a1;
}

template <typename T1>
std::basic_istream<char, T1> &f6(std::basic_istream<char, T1> &a0,
                                 signed char *a1) {
  return a0 >> a1;
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &f7(std::basic_istream<T1, T2> &a0, T1 *a1) {
  return a0 >> a1;
}
