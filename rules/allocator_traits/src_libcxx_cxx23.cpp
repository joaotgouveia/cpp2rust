#include <memory>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2>
std::allocation_result<typename std::allocator_traits<T1>::pointer,
                       typename std::allocator_traits<T1>::size_type>
f10(T2 &a0, typename std::allocator_traits<T1>::size_type a1) {
  return std::allocator_traits<T1>::allocate_at_least(a0, a1);
}
