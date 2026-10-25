#include <type_traits>
#include <utility>

template <typename T1, typename... T2>
bool f12(const std::conjunction<T1, T2...> &a0)
  requires std::is_base_of<std::conjunction<T2...>,
                           std::conjunction<T1, T2...>>::value
{
  return static_cast<bool>(a0);
}

template <typename T1, typename... T2>
bool f13(const std::conjunction<T1, T2...> &a0)
  requires std::is_base_of<std::conjunction<T2...>,
                           std::conjunction<T1, T2...>>::value
{
  return a0.operator()();
}
