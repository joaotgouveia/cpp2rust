#include <algorithm>
#include <ios>
#include <iosfwd>
#include <sstream>
#include <string>
#include <string_view>
#include <utility>

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f32(std::basic_ostringstream<T1, T2, T3> &&a0) {
  return std::move(a0).str();
}

template <typename T1, typename T2, typename T3>
std::basic_string_view<T1, T2>
f33(const std::basic_ostringstream<T1, T2, T3> &a0) {
  return a0.view();
}

template <typename T1, typename T2, typename T3>
void f34(std::basic_ostringstream<T1, T2, T3> &a0,
         std::basic_string<T1, T2, T3> &&a1) {
  return a0.str(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_ostringstream<T1, T2, T3>
f35(const std::basic_string<T1, T2, T4> &a0, const T3 &a1) {
  return std::basic_ostringstream<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<T1, T2, T4>
f36(const std::basic_ostringstream<T1, T2, T3> &a0, const T4 &a1) {
  return a0.template str<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
void f37(std::basic_ostringstream<T1, T2, T3> &a0,
         const std::basic_string<T1, T2, T4> &a1) {
  return a0.template str<T4>(a1);
}
