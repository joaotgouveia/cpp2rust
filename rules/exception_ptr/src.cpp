#include "../common/init.h"
#include "../common/traits.h"
#include <exception>
#include <utility>

std::exception_ptr f1() { return std::current_exception(); }

template <typename T1> std::exception_ptr f2(T1 a0) {
  return std::make_exception_ptr<T1>(a0);
}

void f3(std::exception_ptr a0) { return std::rethrow_exception(std::move(a0)); }
