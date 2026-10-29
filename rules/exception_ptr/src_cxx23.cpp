#include <algorithm>
#include <exception>
#include <utility>

using t1 = std::exception_ptr;

std::exception_ptr f1() { return std::exception_ptr(); }

std::exception_ptr f2(const std::exception_ptr &a0) {
  return std::exception_ptr(a0);
}

std::exception_ptr f3(std::nullptr_t a0) { return std::exception_ptr(a0); }

std::exception_ptr f4(std::exception_ptr &&a0) {
  return std::exception_ptr(std::move(a0));
}

void f5(std::exception_ptr &a0, std::exception_ptr &a1) { return a0.swap(a1); }

bool f6(const std::exception_ptr &a0) { return static_cast<bool>(a0); }

bool f7(const std::exception_ptr &a0, const std::exception_ptr &a1) {
  return operator==(a0, a1);
}

std::exception_ptr f8() { return std::current_exception(); }

template <typename T1> std::exception_ptr f9(T1 a0) {
  return std::make_exception_ptr(std::move(a0));
}

void f10(std::exception_ptr a0) {
  return std::rethrow_exception(std::move(a0));
}

void f11(std::exception_ptr &a0, std::exception_ptr &a1) {
  return std::swap(a0, a1);
}
