#include <algorithm>
#include <ios>
#include <iosfwd>
#include <sstream>
#include <string>
#include <string_view>
#include <utility>

template <typename T1, typename T2, typename T3>
std::basic_string_view<
    typename std::basic_stringstream<T1, T2, T3>::char_type,
    typename std::basic_stringstream<T1, T2, T3>::traits_type>
f44(const std::basic_stringstream<T1, T2, T3> &a0) {
  return a0.view();
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_stringstream<T1, T2, T3>
f45(const std::basic_string<T1, T2, T4> &a0,
    typename std::basic_stringstream<T1, T2, T3>::openmode a1) {
  return std::basic_stringstream<T1, T2, T3>(a0, a1);
}
