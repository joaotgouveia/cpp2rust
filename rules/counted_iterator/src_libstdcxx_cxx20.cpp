#include <algorithm>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1>
std::iter_rvalue_reference_t<T1> f25(const std::counted_iterator<T1> &a0) {
  return iter_move(a0);
}
