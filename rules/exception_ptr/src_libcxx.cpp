#include <cstddef>
#include <exception>
#include <utility>

using t2 = std::exception_ptr;

std::exception_ptr f14() { return std::exception_ptr(); }

std::exception_ptr f15(std::nullptr_t a0) { return std::exception_ptr(a0); }

std::exception_ptr f16(const std::exception_ptr &a0) {
  return std::exception_ptr(a0);
}

std::exception_ptr f17(std::exception_ptr &&a0) {
  return std::exception_ptr(std::move(a0));
}

std::exception_ptr &f18(std::exception_ptr &a0, const std::exception_ptr &a1) {
  return a0.operator=(a1);
}

std::exception_ptr &f19(std::exception_ptr &a0, std::exception_ptr &&a1) {
  return a0.operator=(std::move(a1));
}

bool f20(const std::exception_ptr &a0) { return static_cast<bool>(a0); }

bool f21(const std::exception_ptr &a0, const std::exception_ptr &a1) {
  return operator!=(a0, a1);
}

bool f22(const std::exception_ptr &a0, const std::exception_ptr &a1) {
  return operator==(a0, a1);
}

void f23(std::exception_ptr &a0, std::exception_ptr &a1) {
  return std::swap(a0, a1);
}
