#include "../common/init.h"
#include "../common/traits.h"
#include <memory>
#include <utility>

template <typename T1, typename T2, typename... T3>
auto f2(T2 &a0, T3 &&...a1) {
  return std::inout_ptr<T1, T2, T3...>(a0, std::forward<T3>(a1)...);
}
