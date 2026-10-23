#include <cstddef>
#include <memory>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1>
std::allocation_result<T1 *> f38(std::allocator<T1> &a0, std::size_t a1) {
  return a0.allocate_at_least(a1);
}
