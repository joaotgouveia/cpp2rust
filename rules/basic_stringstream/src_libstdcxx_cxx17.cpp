#include <algorithm>
#include <sstream>
#include <string>
#include <utility>

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3> &
f108(std::basic_stringstream<T1, T2, T3> &a0, std::nullptr_t a1) {
  return a0.operator<<(a1);
}
