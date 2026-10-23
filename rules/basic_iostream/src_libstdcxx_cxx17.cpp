#include <algorithm>
#include <istream>
#include <string>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2>
std::basic_iostream<T1, T2> f97(std::basic_streambuf<T1, T2> *a0) {
  return std::basic_iostream<T1, T2>(a0);
}

template <typename T1, typename T2>
std::basic_iostream<T1, T2> &f98(std::basic_iostream<T1, T2> &a0,
                                 std::nullptr_t a1) {
  return a0.operator<<(a1);
}
