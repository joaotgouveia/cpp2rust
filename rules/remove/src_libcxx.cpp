#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <utility>

template <typename T1, typename T2> T1 f1(T1 a0, T1 a1, const T2 &a2) {
  return std::remove<T1, T2>(a0, a1, a2);
}
