#include <algorithm>
#include <fstream>
#include <ios>
#include <string>
#include <utility>

template <typename T1, typename T2>
std::basic_fstream<T1, T2> &f109(std::basic_fstream<T1, T2> &a0,
                                 std::nullptr_t a1) {
  return a0.operator<<(a1);
}
