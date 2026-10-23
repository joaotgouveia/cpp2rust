#include <type_traits>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1> std::underlying_type_t<T1> f1(T1 a0) {
  return std::to_underlying<T1>(a0);
}
