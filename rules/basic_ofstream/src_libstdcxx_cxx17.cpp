#include <algorithm>
#include <fstream>
#include <ios>
#include <string>
#include <utility>

template <typename T1, typename T2>
std::basic_ofstream<T1, T2> &f70(std::basic_ofstream<T1, T2> &a0,
                                 std::nullptr_t a1) {
  return a0.operator<<(a1);
}
