#include <sstream>
#include <string>
#include <string_view>
#include <utility>

template <typename T1, typename T2, typename T3>
std::basic_ostringstream<T1, T2, T3> f90(unsigned int a0, const T3 &a1) {
  return std::basic_ostringstream<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_ostringstream<T1, T2, T3> f91(std::basic_string<T1, T2, T3> &&a0,
                                         unsigned int a1) {
  return std::basic_ostringstream<T1, T2, T3>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_ostringstream<T1, T2, T3>
f92(const std::basic_string<T1, T2, T4> &a0, unsigned int a1, const T3 &a2) {
  return std::basic_ostringstream<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_ostringstream<T1, T2, T3>
f93(const std::basic_string<T1, T2, T4> &a0, unsigned int a1) {
  return std::basic_ostringstream<T1, T2, T3>(a0, a1);
}
