#include <future>
#include <memory>
#include <utility>

template <
    typename... T1, typename T2,
    std::__enable_if_t<!std::is_same<std::__remove_cvref_t<T2>,
                                     std::packaged_task<void(T1...)>>::value,
                       int>
        T3 = 0,
    typename std::enable_if<
        (!(std::is_same<T2, const std::packaged_task<void(T1...)> &>::value) &&
         !(std::is_same<T2, std::packaged_task<void(T1...)>>::value)),
        int>::type T4 = 0>
std::packaged_task<void(T1...)> f24(T2 &&a0) {
  return std::packaged_task<void(T1...)>(std::forward<T2>(a0));
}

template <
    typename T1, typename... T2, typename T3,
    std::__enable_if_t<!std::is_same<std::__remove_cvref_t<T3>,
                                     std::packaged_task<T1(T2...)>>::value,
                       int>
        T4 = 0,
    typename std::enable_if<
        (!(std::is_same<T3, const std::packaged_task<T1(T2...)> &>::value) &&
         !(std::is_same<T3, std::packaged_task<T1(T2...)>>::value)),
        int>::type T5 = 0>
std::packaged_task<T1(T2...)> f25(T3 &&a0) {
  return std::packaged_task<T1(T2...)>(std::forward<T3>(a0));
}
