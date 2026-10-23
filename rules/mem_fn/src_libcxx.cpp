#include <functional>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2>
auto f1(T1 T2::*a0) -> decltype(std::mem_fn<T1, T2>(a0)) {
  return std::mem_fn<T1, T2>(a0);
}
