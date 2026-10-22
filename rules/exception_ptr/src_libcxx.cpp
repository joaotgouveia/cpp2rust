#include "../common/init.h"
#include "../common/traits.h"
#include <cstddef>
#include <exception>
#include <utility>

using t1 = std::exception_ptr;

std::exception_ptr f4() { return std::exception_ptr(); }

std::exception_ptr f5(std::nullptr_t a0) { return std::exception_ptr(a0); }

std::exception_ptr f6(const std::exception_ptr &a0) {
  return std::exception_ptr(a0);
}

std::exception_ptr f7(std::exception_ptr &&a0) {
  return std::exception_ptr(std::move(a0));
}

std::exception_ptr &f8(std::exception_ptr &a0, const std::exception_ptr &a1) {
  return a0.operator=(a1);
}

std::exception_ptr &f9(std::exception_ptr &a0, std::exception_ptr &&a1) {
  return a0.operator=(std::move(a1));
}

bool f10(const std::exception_ptr &a0) { return static_cast<bool>(a0); }

bool f11(const std::exception_ptr &a0, const std::exception_ptr &a1) {
  return a0 != a1;
}

bool f12(const std::exception_ptr &a0, const std::exception_ptr &a1) {
  return a0 == a1;
}

void f13(std::exception_ptr &a0, std::exception_ptr &a1) {
  return std::swap(a0, a1);
}
