#include "../common/init.h"
#include "../common/traits.h"
#include <string>
#include <utility>

template <typename T1> using t1 = std::fpos<T1>;

template <typename T1> std::fpos<T1> f1() { return std::fpos<T1>(); }

template <typename T1> std::fpos<T1> f2(std::streamoff a0) {
  return std::fpos<T1>(a0);
}

template <typename T1> std::fpos<T1> f3(const std::fpos<T1> &a0) {
  return std::fpos<T1>(a0);
}

template <typename T1>
std::fpos<T1> &f4(std::fpos<T1> &a0, const std::fpos<T1> &a1) {
  return a0.operator=(a1);
}

template <typename T1> std::streamoff f5(const std::fpos<T1> &a0) {
  return static_cast<std::streamoff>(a0);
}

template <typename T1> void f6(std::fpos<T1> &a0, T1 a1) {
  return a0.state(a1);
}

template <typename T1> T1 f7(const std::fpos<T1> &a0) { return a0.state(); }

template <typename T1> std::fpos<T1> &f8(std::fpos<T1> &a0, std::streamoff a1) {
  return a0.operator+=(a1);
}

template <typename T1> std::fpos<T1> &f9(std::fpos<T1> &a0, std::streamoff a1) {
  return a0.operator-=(a1);
}

template <typename T1>
std::fpos<T1> f10(const std::fpos<T1> &a0, std::streamoff a1) {
  return a0.operator+(a1);
}

template <typename T1>
std::fpos<T1> f11(const std::fpos<T1> &a0, std::streamoff a1) {
  return a0.operator-(a1);
}

template <typename T1>
std::streamoff f12(const std::fpos<T1> &a0, const std::fpos<T1> &a1) {
  return a0.operator-(a1);
}
