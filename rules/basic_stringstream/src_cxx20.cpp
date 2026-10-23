#include <algorithm>
#include <ios>
#include <iosfwd>
#include <sstream>
#include <string>
#include <string_view>
#include <utility>

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f38(std::basic_stringstream<T1, T2, T3> &&a0) {
  return std::move(a0).str();
}

template <typename T1, typename T2, typename T3>
std::basic_string_view<T1, T2>
f39(const std::basic_stringstream<T1, T2, T3> &a0) {
  return a0.view();
}

template <typename T1, typename T2, typename T3>
void f40(std::basic_stringstream<T1, T2, T3> &a0,
         std::basic_string<T1, T2, T3> &&a1) {
  return a0.str(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_stringstream<T1, T2, T3> f41(const std::basic_string<T1, T2, T4> &a0,
                                        const T3 &a1) {
  return std::basic_stringstream<T1, T2, T3>(a0, a1);
}
