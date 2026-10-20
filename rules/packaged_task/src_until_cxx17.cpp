#include "../common/init.h"
#include "../common/traits.h"
#include <future>
#include <tuple>
#include <utility>

template <typename T1, typename... T2, typename T3, typename T4>
std::packaged_task<T1(T2...)> f11(std::allocator_arg_t a0, const T4 &a1,
                                  T3 &&a2) {
  return std::packaged_task<T1(T2...)>(std::move(a0), a1, std::forward<T3>(a2));
}

template <typename T1, typename... T2, typename T3>
std::packaged_task<T1(T2...)> f12(std::allocator_arg_t a0, const T3 &a1) {
  return std::packaged_task<T1(T2...)>(std::move(a0), a1);
}

template <typename T1, typename... T2, typename T3>
std::packaged_task<T1(T2...)> f13(std::allocator_arg_t a0, const T3 &a1,
                                  std::packaged_task<T1(T2...)> &&a2) {
  return std::packaged_task<T1(T2...)>(std::move(a0), a1, std::move(a2));
}
