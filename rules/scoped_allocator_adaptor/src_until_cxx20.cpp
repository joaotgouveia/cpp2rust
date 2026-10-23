#include <algorithm>
#include <scoped_allocator>
#include <tuple>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename... T2, typename T3, typename T4, typename... T5,
          typename... T6>
void f13(std::scoped_allocator_adaptor<T1, T2...> &a0, std::pair<T3, T4> *a1,
         std::piecewise_construct_t a2, std::tuple<T5...> a3,
         std::tuple<T6...> a4) {
  return a0.template construct<T3, T4, T5..., T6...>(a1, std::move(a2), a3, a4);
}

template <typename T1, typename... T2, typename T3, typename T4>
void f14(std::scoped_allocator_adaptor<T1, T2...> &a0, std::pair<T3, T4> *a1) {
  return a0.template construct<T3, T4>(a1);
}

template <typename T1, typename... T2, typename T3, typename T4, typename T5,
          typename T6>
void f15(std::scoped_allocator_adaptor<T1, T2...> &a0, std::pair<T3, T4> *a1,
         T5 &&a2, T6 &&a3) {
  return a0.template construct<T3, T4, T5, T6>(a1, std::forward<T5>(a2),
                                               std::forward<T6>(a3));
}

template <typename T1, typename... T2, typename T3, typename T4, typename T5,
          typename T6>
void f16(std::scoped_allocator_adaptor<T1, T2...> &a0, std::pair<T3, T4> *a1,
         const std::pair<T5, T6> &a2) {
  return a0.template construct<T3, T4, T5, T6>(a1, a2);
}

template <typename T1, typename... T2, typename T3, typename T4, typename T5,
          typename T6>
void f17(std::scoped_allocator_adaptor<T1, T2...> &a0, std::pair<T3, T4> *a1,
         std::pair<T5, T6> &&a2) {
  return a0.template construct<T3, T4, T5, T6>(a1, std::move(a2));
}
