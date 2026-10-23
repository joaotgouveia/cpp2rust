#include <algorithm>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2> T2 f1(T1 a0, T1 a1, T2 a2) {
  return std::move_backward<T1, T2>(a0, a1, a2);
}
