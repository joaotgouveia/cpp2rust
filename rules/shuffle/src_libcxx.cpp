#include <algorithm>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2> void f3(T1 a0, T1 a1, T2 &&a2) {
  return std::shuffle<T1, T2>(a0, a1, std::forward<T2>(a2));
}
