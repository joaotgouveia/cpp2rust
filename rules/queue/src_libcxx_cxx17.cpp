#include <queue>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2,
          typename std::enable_if<std::is_swappable_v<T2>, int>::type T3 = 0>
void f44(std::queue<T1, T2> &a0, std::queue<T1, T2> &a1) {
  return std::swap<T1, T2>(a0, a1);
}

template <
    typename T1, typename T2, typename T3,
    typename std::enable_if<std::is_swappable_v<T2> && std::is_swappable_v<T3>,
                            int>::type T4 = 0>
void f45(std::priority_queue<T1, T2, T3> &a0,
         std::priority_queue<T1, T2, T3> &a1) {
  return std::swap<T1, T2, T3>(a0, a1);
}
