#include <ios>
#include <iosfwd>
#include <sstream>
#include <string>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3>
f83(typename std::basic_istringstream<T1, T2, T3>::openmode a0,
    const typename std::basic_istringstream<T1, T2, T3>::allocator_type &a1) {
  return std::basic_istringstream<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3>
f84(std::basic_string<T1, T2, T3> &&a0,
    typename std::basic_istringstream<T1, T2, T3>::openmode a1) {
  return std::basic_istringstream<T1, T2, T3>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f85(std::basic_istringstream<T1, T2, T3> &&a0) {
  return std::move(a0).str();
}

template <typename T1, typename T2, typename T3>
void f86(std::basic_istringstream<T1, T2, T3> &a0,
         std::basic_string<T1, T2, T3> &&a1) {
  return a0.str(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_istringstream<T1, T2, T3>
f87(const std::basic_string<T1, T2, T4> &a0,
    const typename std::basic_istringstream<T1, T2, T3>::allocator_type &a1) {
  return std::basic_istringstream<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_istringstream<T1, T2, T3>
f88(const std::basic_string<T1, T2, T4> &a0,
    typename std::basic_istringstream<T1, T2, T3>::openmode a1,
    const typename std::basic_istringstream<T1, T2, T3>::allocator_type &a2) {
  return std::basic_istringstream<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<T1, T2, T4>
f89(const std::basic_istringstream<T1, T2, T3> &a0, const T4 &a1) {
  return a0.template str<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
void f90(std::basic_istringstream<T1, T2, T3> &a0,
         const std::basic_string<T1, T2, T4> &a1) {
  return a0.template str<T4>(a1);
}
