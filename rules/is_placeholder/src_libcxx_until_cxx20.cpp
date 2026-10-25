#include <functional>
#include <utility>

template <typename T1, typename std::enable_if<
                           std::is_base_of<std::integral_constant<int, 0>,
                                           std::is_placeholder<T1>>::value,
                           int>::type T2 = 0>
int f3(const std::is_placeholder<T1> &a0) {
  return static_cast<int>(a0);
}
