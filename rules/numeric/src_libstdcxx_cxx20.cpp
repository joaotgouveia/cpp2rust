#include <numeric>
#include <type_traits>
#include <utility>

template <typename T1>
std::enable_if_t<std::is_object_v<T1>, T1 *> f17(T1 *a0, T1 *a1) {
  return std::midpoint<T1>(a0, a1);
}
