#include <barrier>
#include <iomanip>
#include <istream>
#include <utility>

template <typename T1> using t5 = std::_Setfill<T1>;

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f4(std::basic_ostream<T1, T2> &a0,
                               decltype(std::setbase(std::declval<int>())) a1) {
  return std::operator<<(a0, std::move(a1));
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &
f6(std::basic_ostream<T1, T2> &a0,
   decltype(std::setprecision(std::declval<int>())) a1) {
  return std::operator<<(a0, std::move(a1));
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f7(std::basic_ostream<T1, T2> &a0,
                               decltype(std::setw(std::declval<int>())) a1) {
  return std::operator<<(a0, std::move(a1));
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f9(std::basic_ostream<T1, T2> &a0,
                               std::_Setfill<T1> a1) {
  return std::operator<<(a0, std::forward<std::_Setfill<T1>>(a1));
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &
f12(std::basic_istream<T1, T2> &a0,
    decltype(std::setbase(std::declval<int>())) a1) {
  return std::operator>>(a0, std::move(a1));
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &
f14(std::basic_istream<T1, T2> &a0,
    decltype(std::setprecision(std::declval<int>())) a1) {
  return std::operator>>(a0, std::move(a1));
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &f15(std::basic_istream<T1, T2> &a0,
                                decltype(std::setw(std::declval<int>())) a1) {
  return std::operator>>(a0, std::move(a1));
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &f17(std::basic_istream<T1, T2> &a0,
                                std::_Setfill<T1> a1) {
  return std::operator>>(a0, std::forward<std::_Setfill<T1>>(a1));
}

template <typename T1> auto f21(const T1 *a0, T1 a1, T1 a2) {
  return std::quoted(a0, std::move(a1), std::move(a2));
}

template <typename T1, typename T2>
auto f22(std::basic_string_view<T1, T2> a0, T1 a1, T1 a2) {
  return std::quoted(std::forward<std::basic_string_view<T1, T2>>(a0),
                     std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3>
auto f23(const std::basic_string<T1, T2, T3> &a0, T1 a1, T1 a2) {
  return std::quoted(a0, std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3>
auto f24(std::basic_string<T1, T2, T3> &a0, T1 a1, T1 a2) {
  return std::quoted(a0, std::move(a1), std::move(a2));
}

decltype(std::setbase(std::declval<int>())) f26(int a0) {
  return std::setbase(a0);
}

template <typename T1> auto f27(T1 a0) { return std::setfill(std::move(a0)); }

decltype(std::setprecision(std::declval<int>())) f29(int a0) {
  return std::setprecision(a0);
}

decltype(std::setw(std::declval<int>())) f30(int a0) { return std::setw(a0); }
