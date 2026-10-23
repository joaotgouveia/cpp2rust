#include <ios>
#include <iosfwd>
#include <sstream>
#include <string>
#include <utility>

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3>
f87(typename std::basic_istringstream<T1, T2, T3>::openmode a0, const T3 &a1) {
  return std::basic_istringstream<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_istringstream<T1, T2, T3>
f88(std::basic_string<T1, T2, T3> &&a0,
    typename std::basic_istringstream<T1, T2, T3>::openmode a1) {
  return std::basic_istringstream<T1, T2, T3>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_istringstream<T1, T2, T3>
f89(const std::basic_string<T1, T2, T4> &a0,
    typename std::basic_istringstream<T1, T2, T3>::openmode a1, const T3 &a2) {
  return std::basic_istringstream<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_istringstream<T1, T2, T3>
f90(const std::basic_string<T1, T2, T4> &a0,
    typename std::basic_istringstream<T1, T2, T3>::openmode a1) {
  return std::basic_istringstream<T1, T2, T3>(a0, a1);
}
