#include <functional>
#include <utility>

template <typename T1>
int f5(const std::is_placeholder<T1> &a0)
  requires std::is_base_of<std::integral_constant<int, 0>,
                           std::is_placeholder<T1>>::value
{
  return static_cast<int>(a0);
}

template <typename T1>
int f6(const std::is_placeholder<T1> &a0)
  requires std::is_base_of<std::integral_constant<int, 0>,
                           std::is_placeholder<T1>>::value
{
  return a0.operator()();
}
