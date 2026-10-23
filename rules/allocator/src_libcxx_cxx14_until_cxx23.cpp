#include <cstddef>
#include <memory>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1>
typename std::allocator<T1>::is_always_equal::value_type
f37(const typename std::allocator<T1>::is_always_equal &a0) {
  return a0.operator()();
}
