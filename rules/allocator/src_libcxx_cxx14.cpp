#include <cstddef>
#include <memory>
#include <utility>

template <typename T1>
typename std::allocator<T1>::propagate_on_container_move_assignment::value_type
f36(const typename std::allocator<T1>::propagate_on_container_move_assignment
        &a0) {
  return a0.operator()();
}

template <typename T1>
typename std::allocator<T1>::is_always_equal::value_type
f37(const typename std::allocator<T1>::is_always_equal &a0) {
  return a0.operator()();
}
