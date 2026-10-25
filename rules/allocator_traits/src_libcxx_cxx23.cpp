#include <memory>
#include <utility>

template <typename T1, typename T2 = T1>
std::allocation_result<typename std::allocator_traits<T1>::pointer,
                       typename std::allocator_traits<T1>::size_type>
f43(T2 &a0, typename std::allocator_traits<T1>::size_type a1) {
  return std::allocator_traits<T1>::allocate_at_least(
      a0, std::forward<typename std::allocator_traits<T1>::size_type>(a1));
}
