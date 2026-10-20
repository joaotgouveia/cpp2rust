#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <sstream>
#include <string>
#include <utility>

template <typename T1, typename T2, typename T3>
std::basic_ostringstream<T1, T2, T3> &
f66(std::basic_ostringstream<T1, T2, T3> &a0, std::nullptr_t a1) {
  return a0.operator<<(a1);
}
