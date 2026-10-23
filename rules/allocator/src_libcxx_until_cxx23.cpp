#include <cstddef>
#include <memory>
#include <utility>

template <typename T1> using t33 = typename std::allocator<T1>::is_always_equal;

template <typename T1>
using t34 = typename std::allocator<T1>::is_always_equal::value_type;

template <typename T1>
using t35 = typename std::allocator<T1>::is_always_equal::type;

template <typename T1>
typename std::allocator<T1>::is_always_equal::value_type
f32(const typename std::allocator<T1>::is_always_equal &a0) {
  return static_cast<typename std::allocator<T1>::is_always_equal::value_type>(
      a0);
}
