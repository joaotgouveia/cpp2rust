#include <cstddef>
#include <exception>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1> std::exception_ptr f1(T1 a0) {
  return std::make_exception_ptr<T1>(a0);
}
