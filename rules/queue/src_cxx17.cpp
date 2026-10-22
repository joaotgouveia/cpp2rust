#include "../common/init.h"
#include "../common/traits.h"
#include <queue>
#include <utility>

template <typename T1, typename T2, typename... T3>
decltype(auto) f16(std::queue<T1, T2> &a0, T3 &&...a1) {
  return a0.template emplace<T3...>(std::forward<T3>(a1)...);
}
