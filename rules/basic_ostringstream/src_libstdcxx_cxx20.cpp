#include <algorithm>
#include <ios>
#include <iosfwd>
#include <sstream>
#include <string>
#include <utility>

template <typename T1, typename T2, typename T3>
std::basic_ostringstream<T1, T2, T3>
f69(typename std::basic_ostringstream<T1, T2, T3>::openmode a0,
    const typename std::basic_ostringstream<T1, T2, T3>::allocator_type &a1) {
  return std::basic_ostringstream<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostringstream<T1, T2, T3>
f70(std::basic_string<T1, T2, T3> &&a0,
    typename std::basic_ostringstream<T1, T2, T3>::openmode a1) {
  return std::basic_ostringstream<T1, T2, T3>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f71(std::basic_ostringstream<T1, T2, T3> &&a0) {
  return std::move(a0).str();
}

template <typename T1, typename T2, typename T3>
void f72(std::basic_ostringstream<T1, T2, T3> &a0,
         std::basic_string<T1, T2, T3> &&a1) {
  return a0.str(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_ostringstream<T1, T2, T3>
f73(const std::basic_string<T1, T2, T4> &a0,
    const typename std::basic_ostringstream<T1, T2, T3>::allocator_type &a1) {
  return std::basic_ostringstream<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_ostringstream<T1, T2, T3>
f74(const std::basic_string<T1, T2, T4> &a0,
    typename std::basic_ostringstream<T1, T2, T3>::openmode a1,
    const typename std::basic_ostringstream<T1, T2, T3>::allocator_type &a2) {
  return std::basic_ostringstream<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<T1, T2, T4>
f75(const std::basic_ostringstream<T1, T2, T3> &a0, const T4 &a1) {
  return a0.template str<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
void f76(std::basic_ostringstream<T1, T2, T3> &a0,
         const std::basic_string<T1, T2, T4> &a1) {
  return a0.template str<T4>(a1);
}
