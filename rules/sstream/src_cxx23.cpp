#include <sstream>
#include <utility>

template <typename T1, typename T2, typename T3>
void f4(std::basic_stringstream<T1, T2, T3> &a0,
        std::basic_stringstream<T1, T2, T3> &a1) {
  return std::swap(a0, a1);
}
