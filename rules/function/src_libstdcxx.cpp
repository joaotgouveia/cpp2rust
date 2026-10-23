#include <algorithm>
#include <exception>
#include <functional>
#include <string>
#include <utility>

template <typename T1, typename... T2, typename T3,
          typename T4 = std::__enable_if_t<
              std::function<T1(T2...)>::template _Callable<T3>::value, void>>
std::function<T1(T2...)> f14(T3 &&a0) {
  return std::function<T1(T2...)>(std::forward<T3>(a0));
}

template <typename T1, typename... T2, typename T3>
std::function<T1(T2...)> &f15(std::function<T1(T2...)> &a0,
                              std::reference_wrapper<T3> a1) {
  return a0.template operator= <T3>(a1);
}
