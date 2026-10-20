#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <ios>
#include <ostream>
#include <streambuf>
#include <string>
#include <utility>

template <typename T1, typename T2>
std::basic_ostream<T1, T2> f59(std::basic_streambuf<T1, T2> *a0) {
  return std::basic_ostream<T1, T2>(a0);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f60(std::basic_ostream<T1, T2> &a0,
                                std::nullptr_t a1) {
  return a0.operator<<(a1);
}
