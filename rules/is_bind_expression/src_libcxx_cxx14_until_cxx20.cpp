#include <functional>
#include <utility>

template <typename T1, typename std::enable_if<
                           std::is_base_of<std::integral_constant<bool, false>,
                                           std::is_bind_expression<T1>>::value,
                           int>::type T2 = 0>
bool f4(const std::is_bind_expression<T1> &a0) {
  return a0.operator()();
}
