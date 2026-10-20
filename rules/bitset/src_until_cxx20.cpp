#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <bitset>
#include <utility>

template <std::size_t T1>
bool f45(const std::bitset<T1> &a0, const std::bitset<T1> &a1) {
  return a0.operator!=(a1);
}
