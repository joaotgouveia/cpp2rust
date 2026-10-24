#include <algorithm>
#include <exception>
#include <utility>

using t1 = std::__exception_ptr::exception_ptr;

std::__exception_ptr::exception_ptr f2() {
  return std::__exception_ptr::exception_ptr();
}

std::__exception_ptr::exception_ptr
f3(const std::__exception_ptr::exception_ptr &a0) {
  return std::__exception_ptr::exception_ptr(a0);
}

std::__exception_ptr::exception_ptr f4(std::nullptr_t a0) {
  return std::__exception_ptr::exception_ptr(a0);
}

std::__exception_ptr::exception_ptr
f5(std::__exception_ptr::exception_ptr &&a0) {
  return std::__exception_ptr::exception_ptr(std::move(a0));
}

std::__exception_ptr::exception_ptr &
f6(std::__exception_ptr::exception_ptr &a0,
   const std::__exception_ptr::exception_ptr &a1) {
  return a0.operator=(a1);
}

std::__exception_ptr::exception_ptr &
f7(std::__exception_ptr::exception_ptr &a0,
   std::__exception_ptr::exception_ptr &&a1) {
  return a0.operator=(std::move(a1));
}

void f8(std::__exception_ptr::exception_ptr &a0,
        std::__exception_ptr::exception_ptr &a1) {
  return a0.swap(a1);
}

bool f9(const std::__exception_ptr::exception_ptr &a0) {
  return static_cast<bool>(a0);
}

bool f10(const std::__exception_ptr::exception_ptr &a0,
         const std::__exception_ptr::exception_ptr &a1) {
  return operator==(a0, a1);
}

std::exception_ptr f11() { return std::current_exception(); }

void f12(std::exception_ptr a0) {
  return std::rethrow_exception(std::move(a0));
}
