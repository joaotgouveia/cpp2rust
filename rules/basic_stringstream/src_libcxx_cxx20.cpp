#include "../common/init.h"
#include "../common/traits.h"
#include <sstream>
#include <string>
#include <string_view>
#include <utility>

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3>
f122(typename std::basic_stringstream<T1, T2, T3>::openmode a0, const T3 &a1) {
  return std::basic_stringstream<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_stringstream<T1, T2, T3>
f123(typename std::basic_stringstream<T1, T2, T3>::string_type &&a0,
     typename std::basic_stringstream<T1, T2, T3>::openmode a1) {
  return std::basic_stringstream<T1, T2, T3>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_stringstream<T1, T2, T3>::string_type
f124(std::basic_stringstream<T1, T2, T3> &&a0) {
  return std::move(a0).str();
}

template <typename T1, typename T2, typename T3>
void f125(std::basic_stringstream<T1, T2, T3> &a0,
          typename std::basic_stringstream<T1, T2, T3>::string_type &&a1) {
  return a0.str(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_stringstream<T1, T2, T3>
f126(const std::basic_string<T1, T2, T4> &a0, const T3 &a1) {
  return std::basic_stringstream<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_stringstream<T1, T2, T3>
f127(const std::basic_string<T1, T2, T4> &a0,
     typename std::basic_stringstream<T1, T2, T3>::openmode a1, const T3 &a2) {
  return std::basic_stringstream<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<typename std::basic_stringstream<T1, T2, T3>::char_type,
                  typename std::basic_stringstream<T1, T2, T3>::traits_type, T4>
f128(const std::basic_stringstream<T1, T2, T3> &a0, const T4 &a1) {
  return a0.template str<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
void f129(
    std::basic_stringstream<T1, T2, T3> &a0,
    const std::basic_string<
        typename std::basic_stringstream<T1, T2, T3>::char_type,
        typename std::basic_stringstream<T1, T2, T3>::traits_type, T4> &a1) {
  return a0.template str<T4>(a1);
}
