#include <algorithm>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2, typename T3, typename T4>
T2 f1(T1 a0, T1 a1, T2 a2, T3 a3, T4 &&a4) {
  return std::sample<T1, T2, T3, T4>(a0, a1, a2, a3, std::forward<T4>(a4));
}
