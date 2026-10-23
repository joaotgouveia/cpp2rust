#include <algorithm>
#include <queue>
#include <ranges>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2, typename T3, typename T4>
void f13(std::priority_queue<T1, T2, T3> &a0, T4 &&a1) {
  return a0.template push_range<T4>(std::forward<T4>(a1));
}
