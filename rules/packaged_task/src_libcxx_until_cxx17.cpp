#include "../common/init.h"
#include "../common/traits.h"
#include <future>
#include <memory>
#include <utility>

template <typename... T1, typename T2, typename T3,
          typename std::enable_if<
              !std::is_same<typename std::remove_cv<
                                typename std::remove_reference<T2>::type>::type,
                            std::packaged_task<void(T1...)>>::value,
              int>::type T4 = 0>
std::packaged_task<void(T1...)> f25(std::allocator_arg_t a0, const T3 &a1,
                                    T2 &&a2) {
  return std::packaged_task<void(T1...)>(std::move(a0), a1,
                                         std::forward<T2>(a2));
}

template <typename T1, typename... T2, typename T3, typename T4,
          typename std::enable_if<
              !std::is_same<typename std::remove_cv<
                                typename std::remove_reference<T3>::type>::type,
                            std::packaged_task<T1(T2...)>>::value,
              int>::type T5 = 0>
std::packaged_task<T1(T2...)> f26(std::allocator_arg_t a0, const T4 &a1,
                                  T3 &&a2) {
  return std::packaged_task<T1(T2...)>(std::move(a0), a1, std::forward<T3>(a2));
}
