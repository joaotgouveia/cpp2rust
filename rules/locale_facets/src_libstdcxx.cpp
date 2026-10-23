#include <ios>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1> bool f1(T1 a0, const std::locale &a1) {
  return std::isalnum<T1>(a0, a1);
}

template <typename T1> bool f2(T1 a0, const std::locale &a1) {
  return std::isalpha<T1>(a0, a1);
}

template <typename T1> bool f3(T1 a0, const std::locale &a1) {
  return std::isblank<T1>(a0, a1);
}

template <typename T1> bool f4(T1 a0, const std::locale &a1) {
  return std::iscntrl<T1>(a0, a1);
}

template <typename T1> bool f5(T1 a0, const std::locale &a1) {
  return std::isdigit<T1>(a0, a1);
}

template <typename T1> bool f6(T1 a0, const std::locale &a1) {
  return std::isgraph<T1>(a0, a1);
}

template <typename T1> bool f7(T1 a0, const std::locale &a1) {
  return std::islower<T1>(a0, a1);
}

template <typename T1> bool f8(T1 a0, const std::locale &a1) {
  return std::isprint<T1>(a0, a1);
}

template <typename T1> bool f9(T1 a0, const std::locale &a1) {
  return std::ispunct<T1>(a0, a1);
}

template <typename T1> bool f10(T1 a0, const std::locale &a1) {
  return std::isspace<T1>(a0, a1);
}

template <typename T1> bool f11(T1 a0, const std::locale &a1) {
  return std::isupper<T1>(a0, a1);
}

template <typename T1> bool f12(T1 a0, const std::locale &a1) {
  return std::isxdigit<T1>(a0, a1);
}

template <typename T1> T1 f13(T1 a0, const std::locale &a1) {
  return std::tolower<T1>(a0, a1);
}

template <typename T1> T1 f14(T1 a0, const std::locale &a1) {
  return std::toupper<T1>(a0, a1);
}
