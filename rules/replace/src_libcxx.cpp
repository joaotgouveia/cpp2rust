#include <algorithm>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2>
void f1(T1 a0, T1 a1, const T2 &a2, const T2 &a3) {
  return std::replace<T1, T2>(a0, a1, a2, a3);
}
