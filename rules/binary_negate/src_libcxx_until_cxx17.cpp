#include "../common/init.h"
#include "../common/traits.h"
#include <functional>
#include <utility>

template <typename T1> std::binary_negate<T1> f3(const T1 &a0) {
  return std::not2<T1>(a0);
}
