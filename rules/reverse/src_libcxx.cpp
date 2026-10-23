#include <algorithm>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1> void f1(T1 a0, T1 a1) {
  return std::reverse<T1>(a0, a1);
}
