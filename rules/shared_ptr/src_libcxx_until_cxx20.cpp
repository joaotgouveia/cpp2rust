#include <atomic>
#include <cstddef>
#include <memory>
#include <utility>

template <typename T1> bool f126(const std::shared_ptr<T1> &a0) {
  return a0.unique();
}

template <
    typename T1, typename T2,
    std::__enable_if_t<std::__compatible_with_v<T2, T1>, int> T3 = 0,
    typename std::enable_if<(!(std::is_same<T2, T1>::value)), int>::type T4 = 0>
std::shared_ptr<T1> f127(const std::shared_ptr<T2> &a0) {
  return std::shared_ptr<T1>(a0);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<std::__compatible_with_v<T2, T1>, int> T3 = 0,
    typename std::enable_if<(!(std::is_same<T2, T1>::value)), int>::type T4 = 0>
std::shared_ptr<T1> f128(std::shared_ptr<T2> &&a0) {
  return std::shared_ptr<T1>(std::move(a0));
}

template <
    typename T1, typename T2,
    std::__enable_if_t<std::__compatible_with_v<T2, T1>, int> T3 = 0,
    typename std::enable_if<(!(std::is_same<T2, T1>::value)), int>::type T4 = 0>
std::shared_ptr<T1> &f129(std::shared_ptr<T1> &a0,
                          const std::shared_ptr<T2> &a1) {
  return a0.template operator= <T2>(a1);
}

template <
    typename T1, typename T2,
    std::__enable_if_t<std::__compatible_with_v<T2, T1>, int> T3 = 0,
    typename std::enable_if<(!(std::is_same<T2, T1>::value)), int>::type T4 = 0>
std::shared_ptr<T1> &f130(std::shared_ptr<T1> &a0, std::shared_ptr<T2> &&a1) {
  return a0.template operator= <T2>(std::move(a1));
}
