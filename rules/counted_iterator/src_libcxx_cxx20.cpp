#include <iterator>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1> decltype(auto) f26(const std::counted_iterator<T1> &a0) {
  return iter_move(a0);
}
