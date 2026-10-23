#include <memory>
#include <utility>

template <typename T1>
typename std::allocator_traits<T1>::pointer
f8(typename std::allocator_traits<T1>::allocator_type &a0,
   typename std::allocator_traits<T1>::size_type a1) {
  return std::allocator_traits<T1>::allocate(a0, a1);
}

template <typename T1>
void f9(typename std::allocator_traits<T1>::allocator_type &a0,
        typename std::allocator_traits<T1>::pointer a1,
        typename std::allocator_traits<T1>::size_type a2) {
  return std::allocator_traits<T1>::deallocate(a0, a1, a2);
}

template <
    typename T1, typename T2 = T1,
    std::__enable_if_t<
        std::__has_allocate_hint_v<
            T1, typename std::__allocator_traits_base<T1>::size_type,
            typename std::__allocator_traits_base<T1>::const_void_pointer>,
        int>
        T3 = 0>
typename std::allocator_traits<T1>::pointer
f10(typename std::allocator_traits<T1>::allocator_type &a0,
    typename std::allocator_traits<T1>::size_type a1,
    typename std::allocator_traits<T1>::const_void_pointer a2) {
  return std::allocator_traits<T1>::allocate(a0, a1, a2);
}

template <
    typename T1, typename T2 = T1,
    std::__enable_if_t<
        !std::__has_allocate_hint_v<
            T1, typename std::__allocator_traits_base<T1>::size_type,
            typename std::__allocator_traits_base<T1>::const_void_pointer>,
        int>
        T3 = 0>
typename std::allocator_traits<T1>::pointer
f11(typename std::allocator_traits<T1>::allocator_type &a0,
    typename std::allocator_traits<T1>::size_type a1,
    typename std::allocator_traits<T1>::const_void_pointer a2) {
  return std::allocator_traits<T1>::allocate(a0, a1, a2);
}

template <typename T1, typename T2, typename... T3,
          std::__enable_if_t<
              std::__has_construct_v<
                  typename std::__allocator_traits_base<T1>::allocator_type,
                  T2 *, T3...>,
              int>
              T4 = 0>
void f12(typename std::allocator_traits<T1>::allocator_type &a0, T2 *a1,
         T3 &&...a2) {
  return std::allocator_traits<T1>::construct(a0, a1, std::forward<T3>(a2)...);
}

template <typename T1, typename T2, typename... T3,
          std::__enable_if_t<
              !std::__has_construct_v<
                  typename std::__allocator_traits_base<T1>::allocator_type,
                  T2 *, T3...>,
              int>
              T4 = 0>
void f13(typename std::allocator_traits<T1>::allocator_type &a0, T2 *a1,
         T3 &&...a2) {
  return std::allocator_traits<T1>::construct(a0, a1, std::forward<T3>(a2)...);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<
        std::__has_destroy_v<
            typename std::__allocator_traits_base<T1>::allocator_type, T2 *>,
        int>
        T3 = 0>
void f14(typename std::allocator_traits<T1>::allocator_type &a0, T2 *a1) {
  return std::allocator_traits<T1>::destroy(a0, a1);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<
        !std::__has_destroy_v<
            typename std::__allocator_traits_base<T1>::allocator_type, T2 *>,
        int>
        T3 = 0>
void f15(typename std::allocator_traits<T1>::allocator_type &a0, T2 *a1) {
  return std::allocator_traits<T1>::destroy(a0, a1);
}

template <typename T1, typename T2 = T1,
          std::__enable_if_t<std::__has_max_size_v<T1 const>, int> T3 = 0>
typename std::allocator_traits<T1>::size_type
f16(const typename std::allocator_traits<T1>::allocator_type &a0) {
  return std::allocator_traits<T1>::max_size(a0);
}

template <typename T1, typename T2 = T1,
          std::__enable_if_t<!std::__has_max_size_v<T1 const>, int> T3 = 0>
typename std::allocator_traits<T1>::size_type
f17(const typename std::allocator_traits<T1>::allocator_type &a0) {
  return std::allocator_traits<T1>::max_size(a0);
}

template <typename T1, typename T2 = T1,
          std::__enable_if_t<
              std::__has_select_on_container_copy_construction_v<T1 const>, int>
              T3 = 0>
typename std::allocator_traits<T1>::allocator_type
f18(const typename std::allocator_traits<T1>::allocator_type &a0) {
  return std::allocator_traits<T1>::select_on_container_copy_construction(a0);
}

template <
    typename T1, typename T2 = T1,
    std::__enable_if_t<
        !std::__has_select_on_container_copy_construction_v<T1 const>, int>
        T3 = 0>
typename std::allocator_traits<T1>::allocator_type
f19(const typename std::allocator_traits<T1>::allocator_type &a0) {
  return std::allocator_traits<T1>::select_on_container_copy_construction(a0);
}
