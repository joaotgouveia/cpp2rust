#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <ios>
#include <iosfwd>
#include <sstream>
#include <string>
#include <utility>

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3>
f105(typename std::basic_stringstream<T1, T2, T3>::openmode a0,
     const typename std::basic_stringstream<T1, T2, T3>::allocator_type &a1) {
  return std::basic_stringstream<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3>
f106(std::basic_string<T1, T2, T3> &&a0,
     typename std::basic_stringstream<T1, T2, T3>::openmode a1) {
  return std::basic_stringstream<T1, T2, T3>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f107(std::basic_stringstream<T1, T2, T3> &&a0) {
  return std::move(a0).str();
}

template <typename T1, typename T2, typename T3>
std::basic_string_view<
    typename std::basic_stringstream<T1, T2, T3>::char_type,
    typename std::basic_stringstream<T1, T2, T3>::traits_type>
f108(const std::basic_stringstream<T1, T2, T3> &a0) {
  return a0.view();
}

template <typename T1, typename T2, typename T3>
void f109(std::basic_stringstream<T1, T2, T3> &a0,
          std::basic_string<T1, T2, T3> &&a1) {
  return a0.str(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_stringstream<T1, T2, T3>
f110(const std::basic_string<T1, T2, T4> &a0,
     const typename std::basic_stringstream<T1, T2, T3>::allocator_type &a1) {
  return std::basic_stringstream<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_stringstream<T1, T2, T3>
f111(const std::basic_string<T1, T2, T4> &a0,
     typename std::basic_stringstream<T1, T2, T3>::openmode a1,
     const typename std::basic_stringstream<T1, T2, T3>::allocator_type &a2) {
  return std::basic_stringstream<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_stringstream<T1, T2, T3>
f112(const std::basic_string<T1, T2, T4> &a0,
     typename std::basic_stringstream<T1, T2, T3>::openmode a1) {
  return std::basic_stringstream<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<T1, T2, T4>
f113(const std::basic_stringstream<T1, T2, T3> &a0, const T4 &a1) {
  return a0.template str<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
void f114(std::basic_stringstream<T1, T2, T3> &a0,
          const std::basic_string<T1, T2, T4> &a1) {
  return a0.template str<T4>(a1);
}
