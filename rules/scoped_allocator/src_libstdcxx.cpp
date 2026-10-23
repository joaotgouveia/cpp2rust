#include <scoped_allocator>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2, typename... T3>
bool f2(const std::scoped_allocator_adaptor<T1, T3...> &a0,
        const std::scoped_allocator_adaptor<T2, T3...> &a1) {
  return a0 == a1;
}
