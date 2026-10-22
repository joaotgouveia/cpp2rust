#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <chrono>
#include <utility>

template <std::size_t T1, typename T2> T2 *f2(T2 *a0) {
  return std::assume_aligned<T1, T2>(a0);
}
