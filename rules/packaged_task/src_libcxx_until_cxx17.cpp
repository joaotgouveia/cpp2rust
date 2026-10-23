#include <future>
#include <memory>
#include <utility>

template <
    typename... T1, typename T2, typename T3,
    std::__enable_if_t<!std::is_same<std::__remove_cvref_t<T2>,
                                     std::packaged_task<void(T1...)>>::value,
                       int>
        T4 = 0>
std::packaged_task<void(T1...)> f26(std::allocator_arg_t a0, const T3 &a1,
                                    T2 &&a2) {
  return std::packaged_task<void(T1...)>(std::move(a0), a1,
                                         std::forward<T2>(a2));
}

template <
    typename T1, typename... T2, typename T3, typename T4,
    std::__enable_if_t<!std::is_same<std::__remove_cvref_t<T3>,
                                     std::packaged_task<T1(T2...)>>::value,
                       int>
        T5 = 0>
std::packaged_task<T1(T2...)> f27(std::allocator_arg_t a0, const T4 &a1,
                                  T3 &&a2) {
  return std::packaged_task<T1(T2...)>(std::move(a0), a1, std::forward<T3>(a2));
}
