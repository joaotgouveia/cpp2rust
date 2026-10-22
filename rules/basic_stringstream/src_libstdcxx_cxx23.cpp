#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <ios>
#include <iosfwd>
#include <sstream>
#include <string>
#include <utility>

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3> &
f115(std::basic_stringstream<T1, T2, T3> &a0, const volatile void *a1) {
  return a0.operator<<(a1);
}
