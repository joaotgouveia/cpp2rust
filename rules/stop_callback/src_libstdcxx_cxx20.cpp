#include <stop_token>
#include <utility>

template <typename T1, typename T2,
          std::enable_if_t<std::is_constructible_v<T1, T2>, int> T3 = 0>
std::stop_callback<T1> f1(const std::stop_token &a0, T2 &&a1) {
  return std::stop_callback<T1>(a0, std::forward<T2>(a1));
}

template <typename T1, typename T2,
          std::enable_if_t<std::is_constructible_v<T1, T2>, int> T3 = 0>
std::stop_callback<T1> f2(std::stop_token &&a0, T2 &&a1) {
  return std::stop_callback<T1>(std::move(a0), std::forward<T2>(a1));
}
