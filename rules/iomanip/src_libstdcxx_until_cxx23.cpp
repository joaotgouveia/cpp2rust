#include <iomanip>
#include <ios>
#include <istream>
#include <ostream>
#include <utility>

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &
f9(std::basic_ostream<T1, T2> &a0,
   decltype(std::resetiosflags(std::declval<std::ios_base::fmtflags>())) a1) {
  return a0 << std::move(a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &
f10(std::basic_ostream<T1, T2> &a0,
    decltype(std::setbase(std::declval<int>())) a1) {
  return a0 << std::move(a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &
f11(std::basic_ostream<T1, T2> &a0,
    decltype(std::setiosflags(std::declval<std::ios_base::fmtflags>())) a1) {
  return a0 << std::move(a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &
f12(std::basic_ostream<T1, T2> &a0,
    decltype(std::setprecision(std::declval<int>())) a1) {
  return a0 << std::move(a1);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f13(std::basic_ostream<T1, T2> &a0,
                                decltype(std::setw(std::declval<int>())) a1) {
  return a0 << std::move(a1);
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &
f14(std::basic_istream<T1, T2> &a0,
    decltype(std::resetiosflags(std::declval<std::ios_base::fmtflags>())) a1) {
  return a0 >> std::move(a1);
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &
f15(std::basic_istream<T1, T2> &a0,
    decltype(std::setbase(std::declval<int>())) a1) {
  return a0 >> std::move(a1);
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &
f16(std::basic_istream<T1, T2> &a0,
    decltype(std::setiosflags(std::declval<std::ios_base::fmtflags>())) a1) {
  return a0 >> std::move(a1);
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &
f17(std::basic_istream<T1, T2> &a0,
    decltype(std::setprecision(std::declval<int>())) a1) {
  return a0 >> std::move(a1);
}

template <typename T1, typename T2>
std::basic_istream<T1, T2> &f18(std::basic_istream<T1, T2> &a0,
                                decltype(std::setw(std::declval<int>())) a1) {
  return a0 >> std::move(a1);
}

decltype(std::resetiosflags(std::declval<std::ios_base::fmtflags>()))
f19(std::ios_base::fmtflags a0) {
  return std::resetiosflags(a0);
}

decltype(std::setbase(std::declval<int>())) f20(int a0) {
  return std::setbase(a0);
}

decltype(std::setiosflags(std::declval<std::ios_base::fmtflags>()))
f21(std::ios_base::fmtflags a0) {
  return std::setiosflags(a0);
}

decltype(std::setprecision(std::declval<int>())) f22(int a0) {
  return std::setprecision(a0);
}

decltype(std::setw(std::declval<int>())) f23(int a0) { return std::setw(a0); }
