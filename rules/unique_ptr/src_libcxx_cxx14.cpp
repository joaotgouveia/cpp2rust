#include <cstddef>
#include <memory>
#include <utility>

template <typename T1,
          std::enable_if_t<std::__is_unbounded_array_v<T1>, int> T2 = 0>
std::unique_ptr<T1> f84(std::size_t a0) {
  return std::make_unique<T1>(a0);
}

template <typename T1, typename... T2,
          std::enable_if_t<!std::is_array<T1>::value, int> T3 = 0>
std::unique_ptr<T1> f85(T2 &&...a0) {
  return std::make_unique<T1, T2...>(std::forward<T2>(a0)...);
}
