#include <ios>
#include <utility>

template <typename T1> bool f1(T1 a0, const std::locale &a1) {
  return std::isalnum(std::move(a0), a1);
}

template <typename T1> bool f2(T1 a0, const std::locale &a1) {
  return std::isalpha(std::move(a0), a1);
}

template <typename T1> bool f3(T1 a0, const std::locale &a1) {
  return std::isblank(std::move(a0), a1);
}

template <typename T1> bool f4(T1 a0, const std::locale &a1) {
  return std::iscntrl(std::move(a0), a1);
}

template <typename T1> bool f5(T1 a0, const std::locale &a1) {
  return std::isdigit(std::move(a0), a1);
}

template <typename T1> bool f6(T1 a0, const std::locale &a1) {
  return std::isgraph(std::move(a0), a1);
}

template <typename T1> bool f7(T1 a0, const std::locale &a1) {
  return std::islower(std::move(a0), a1);
}

template <typename T1> bool f8(T1 a0, const std::locale &a1) {
  return std::isprint(std::move(a0), a1);
}

template <typename T1> bool f9(T1 a0, const std::locale &a1) {
  return std::ispunct(std::move(a0), a1);
}

template <typename T1> bool f10(T1 a0, const std::locale &a1) {
  return std::isspace(std::move(a0), a1);
}

template <typename T1> bool f11(T1 a0, const std::locale &a1) {
  return std::isupper(std::move(a0), a1);
}

template <typename T1> bool f12(T1 a0, const std::locale &a1) {
  return std::isxdigit(std::move(a0), a1);
}

template <typename T1> T1 f13(T1 a0, const std::locale &a1) {
  return std::tolower(std::move(a0), a1);
}

template <typename T1> T1 f14(T1 a0, const std::locale &a1) {
  return std::toupper(std::move(a0), a1);
}
