#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <utility>

template <typename T1, typename T2, typename T3> T3 f1(T1 a0, T2 a1, T3 a2) {
  return std::copy_n<T1, T2, T3>(a0, a1, a2);
}
