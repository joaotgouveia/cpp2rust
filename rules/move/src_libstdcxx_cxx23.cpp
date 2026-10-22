#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <utility>

template <typename T1, typename T2> auto f7(T2 &&a0) {
  return std::forward_like<T1, T2>(std::forward<T2>(a0));
}
