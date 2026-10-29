#include <barrier>
#include <iomanip>
#include <istream>
#include <utility>

template <typename T1> using t1 = std::_Get_money<T1>;

template <typename T1> using t2 = std::_Get_time<T1>;

template <typename T1> using t3 = std::_Put_money<T1>;

template <typename T1> using t4 = std::_Put_time<T1>;

template <typename T1> using t5 = std::_Setfill<T1>;

template <typename T1> std::_Get_money<T1> f1(T1 &a0, bool a1) {
  return std::get_money(a0, a1);
}

template <typename T1> std::_Get_time<T1> f2(std::tm *a0, const T1 *a1) {
  return std::get_time(a0, a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &
f3(std::basic_ostream<T1, T2> &a0,
   decltype(std::resetiosflags(std::declval<std::ios_base::fmtflags>())) a1) {
  return std::operator<<(a0, std::move(a1));
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f4(std::basic_ostream<T1, T2> &a0,
                               decltype(std::setbase(std::declval<int>())) a1) {
  return std::operator<<(a0, std::move(a1));
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &
f5(std::basic_ostream<T1, T2> &a0,
   decltype(std::setiosflags(std::declval<std::ios_base::fmtflags>())) a1) {
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
std::basic_ostream<T1, T2> &f8(std::basic_ostream<T1, T2> &a0,
                               std::_Put_time<T1> a1) {
  return std::operator<<(a0, std::forward<std::_Put_time<T1>>(a1));
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f9(std::basic_ostream<T1, T2> &a0,
                               std::_Setfill<T1> a1) {
  return std::operator<<(a0, std::forward<std::_Setfill<T1>>(a1));
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<T1, T2> &f10(std::basic_ostream<T1, T2> &a0,
                                std::_Put_money<T3> a1) {
  return std::operator<<(a0, std::forward<std::_Put_money<T3>>(a1));
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &
f11(std::basic_istream<T1, T2> &a0,
    decltype(std::resetiosflags(std::declval<std::ios_base::fmtflags>())) a1) {
  return std::operator>>(a0, std::move(a1));
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &
f12(std::basic_istream<T1, T2> &a0,
    decltype(std::setbase(std::declval<int>())) a1) {
  return std::operator>>(a0, std::move(a1));
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &
f13(std::basic_istream<T1, T2> &a0,
    decltype(std::setiosflags(std::declval<std::ios_base::fmtflags>())) a1) {
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
std::basic_istream<T1, T2> &f16(std::basic_istream<T1, T2> &a0,
                                std::_Get_time<T1> a1) {
  return std::operator>>(a0, std::forward<std::_Get_time<T1>>(a1));
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &f17(std::basic_istream<T1, T2> &a0,
                                std::_Setfill<T1> a1) {
  return std::operator>>(a0, std::forward<std::_Setfill<T1>>(a1));
}

template <typename T1, typename T2, typename T3>
std::basic_istream<T1, T2> &f18(std::basic_istream<T1, T2> &a0,
                                std::_Get_money<T3> a1) {
  return std::operator>>(a0, std::forward<std::_Get_money<T3>>(a1));
}

template <typename T1> std::_Put_money<T1> f19(const T1 &a0, bool a1) {
  return std::put_money(a0, a1);
}

template <typename T1> std::_Put_time<T1> f20(const std::tm *a0, const T1 *a1) {
  return std::put_time(a0, a1);
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

decltype(std::resetiosflags(std::declval<std::ios_base::fmtflags>()))
f25(std::ios_base::fmtflags a0) {
  return std::resetiosflags(a0);
}

decltype(std::setbase(std::declval<int>())) f26(int a0) {
  return std::setbase(a0);
}

template <typename T1> std::_Setfill<T1> f27(T1 a0) {
  return std::setfill(std::move(a0));
}

decltype(std::setiosflags(std::declval<std::ios_base::fmtflags>()))
f28(std::ios_base::fmtflags a0) {
  return std::setiosflags(a0);
}

decltype(std::setprecision(std::declval<int>())) f29(int a0) {
  return std::setprecision(a0);
}

decltype(std::setw(std::declval<int>())) f30(int a0) { return std::setw(a0); }
