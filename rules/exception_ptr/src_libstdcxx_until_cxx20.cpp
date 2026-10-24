#include <algorithm>
#include <exception>
#include <utility>

bool f13(const std::__exception_ptr::exception_ptr &a0,
         const std::__exception_ptr::exception_ptr &a1) {
  return operator!=(a0, a1);
}
