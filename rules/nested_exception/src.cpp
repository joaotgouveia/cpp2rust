#include "../common/init.h"
#include "../common/traits.h"
#include <exception>
#include <utility>

using t1 = std::nested_exception;

std::nested_exception f1() { return std::nested_exception(); }

std::nested_exception f2(const std::nested_exception &a0) {
  return std::nested_exception(a0);
}

std::nested_exception &f3(std::nested_exception &a0,
                          const std::nested_exception &a1) {
  return a0.operator=(a1);
}

void f4(const std::nested_exception &a0) { return a0.rethrow_nested(); }

std::exception_ptr f5(const std::nested_exception &a0) {
  return a0.nested_ptr();
}

template <typename T1> void f6(T1 &&a0) {
  return std::throw_with_nested<T1>(std::forward<T1>(a0));
}
