#include "../common/init.h"
#include "../common/traits.h"
#include <scoped_allocator>
#include <tuple>
#include <utility>

template <typename T1, typename... T2>
typename std::scoped_allocator_adaptor<
    T1, T2...>::propagate_on_container_copy_assignment::value_type
f32(const typename std::scoped_allocator_adaptor<
    T1, T2...>::propagate_on_container_copy_assignment &a0) {
  return a0.operator()();
}

template <typename T1, typename... T2>
typename std::scoped_allocator_adaptor<
    T1, T2...>::propagate_on_container_move_assignment::value_type
f33(const typename std::scoped_allocator_adaptor<
    T1, T2...>::propagate_on_container_move_assignment &a0) {
  return a0.operator()();
}

template <typename T1, typename... T2>
typename std::scoped_allocator_adaptor<
    T1, T2...>::propagate_on_container_swap::value_type
f34(const typename std::scoped_allocator_adaptor<
    T1, T2...>::propagate_on_container_swap &a0) {
  return a0.operator()();
}

template <typename T1, typename... T2>
typename std::scoped_allocator_adaptor<T1, T2...>::is_always_equal::value_type
f35(const typename std::scoped_allocator_adaptor<T1, T2...>::is_always_equal
        &a0) {
  return a0.operator()();
}
