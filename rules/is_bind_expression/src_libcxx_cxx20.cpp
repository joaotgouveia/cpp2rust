#include <functional>
#include <utility>

template <typename T1>
bool f5(const std::is_bind_expression<T1> &a0)
  requires std::is_base_of<std::integral_constant<bool, false>,
                           std::is_bind_expression<T1>>::value
{
  return static_cast<bool>(a0);
}

template <typename T1>
bool f6(const std::is_bind_expression<T1> &a0)
  requires std::is_base_of<std::integral_constant<bool, false>,
                           std::is_bind_expression<T1>>::value
{
  return a0.operator()();
}
