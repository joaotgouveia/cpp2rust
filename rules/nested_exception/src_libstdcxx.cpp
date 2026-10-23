#include <exception>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1> void f7(const T1 &a0) {
  return std::rethrow_if_nested<T1>(a0);
}
