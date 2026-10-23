#include <functional>
#include <type_traits>
#include <utility>

template <typename T1, typename... T2>
std::invoke_result_t<T1, T2...> f4(T1 &&a0, T2 &&...a1) {
  return std::invoke<T1, T2...>(std::forward<T1>(a0), std::forward<T2>(a1)...);
}

template <typename T1> auto f5(T1 &&a0) {
  return std::not_fn<T1>(std::forward<T1>(a0));
}
