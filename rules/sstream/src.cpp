#include "../common/init.h"
#include "../common/traits.h"
#include <sstream>
#include <utility>

template <typename T1, typename T2, typename T3>
void f1(std::basic_istringstream<T1, T2, T3> &a0,
        std::basic_istringstream<T1, T2, T3> &a1) {
  return std::swap<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
void f2(std::basic_ostringstream<T1, T2, T3> &a0,
        std::basic_ostringstream<T1, T2, T3> &a1) {
  return std::swap<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
void f3(std::basic_stringbuf<T1, T2, T3> &a0,
        std::basic_stringbuf<T1, T2, T3> &a1) {
  return std::swap<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
void f4(std::basic_stringstream<T1, T2, T3> &a0,
        std::basic_stringstream<T1, T2, T3> &a1) {
  return std::swap<T1, T2, T3>(a0, a1);
}
