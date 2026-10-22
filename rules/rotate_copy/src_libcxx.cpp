#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <utility>

template <typename T1, typename T2> T2 f1(T1 a0, T1 a1, T1 a2, T2 a3) {
  return std::rotate_copy<T1, T2>(a0, a1, a2, a3);
}
