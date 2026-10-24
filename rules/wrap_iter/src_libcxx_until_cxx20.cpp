#include <iterator>
#include <utility>

template <typename T1, typename T2>
bool f18(const std::__wrap_iter<T1> &a0, const std::__wrap_iter<T2> &a1) {
  return operator<=(a0, a1);
}

template <typename T1>
bool f19(const std::__wrap_iter<T1> &a0, const std::__wrap_iter<T1> &a1) {
  return operator<=(a0, a1);
}

template <typename T1, typename T2>
bool f20(const std::__wrap_iter<T1> &a0, const std::__wrap_iter<T2> &a1) {
  return operator>=(a0, a1);
}

template <typename T1>
bool f21(const std::__wrap_iter<T1> &a0, const std::__wrap_iter<T1> &a1) {
  return operator>=(a0, a1);
}

template <typename T1, typename T2>
bool f22(const std::__wrap_iter<T1> &a0, const std::__wrap_iter<T2> &a1) {
  return operator>(a0, a1);
}

template <typename T1>
bool f23(const std::__wrap_iter<T1> &a0, const std::__wrap_iter<T1> &a1) {
  return operator>(a0, a1);
}

template <typename T1, typename T2>
bool f24(const std::__wrap_iter<T1> &a0, const std::__wrap_iter<T2> &a1) {
  return operator!=(a0, a1);
}

template <typename T1>
bool f25(const std::__wrap_iter<T1> &a0, const std::__wrap_iter<T1> &a1) {
  return operator!=(a0, a1);
}

template <typename T1, typename T2>
bool f26(const std::__wrap_iter<T1> &a0, const std::__wrap_iter<T2> &a1) {
  return operator<(a0, a1);
}

template <typename T1>
bool f27(const std::__wrap_iter<T1> &a0, const std::__wrap_iter<T1> &a1) {
  return operator<(a0, a1);
}
