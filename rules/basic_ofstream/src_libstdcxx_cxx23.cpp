#include <algorithm>
#include <fstream>
#include <ios>
#include <iosfwd>
#include <type_traits>
#include <utility>

template <typename T1, typename T2>
std::basic_ofstream<T1, T2> &f72(std::basic_ofstream<T1, T2> &a0,
                                 const volatile void *a1) {
  return a0.operator<<(a1);
}
