#include "../common/init.h"
#include "../common/traits.h"
#include <ranges>
#include <utility>

template <typename T1, typename T2> decltype(auto) f1(T1 &&a0, T2 &&a1) {
  return std::forward<T1>(a0) | std::forward<T2>(a1);
}
