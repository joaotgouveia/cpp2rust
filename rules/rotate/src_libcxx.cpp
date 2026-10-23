#include <algorithm>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1> T1 f1(T1 a0, T1 a1, T1 a2) {
  return std::rotate<T1>(a0, a1, a2);
}
