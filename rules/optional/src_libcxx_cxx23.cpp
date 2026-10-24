#include <compare>
#include <initializer_list>
#include <optional>
#include <type_traits>
#include <utility>

template <
    typename T1, typename T2, typename T3, typename... T4,
    std::enable_if_t<
        std::_IsSame<T2, std::__optional_construct_from_invoke_tag>::value, int>
        T5 = 0>
std::optional<T1> f628(T2 a0, T3 &&a1, T4 &&...a2) {
  return std::optional<T1>(a0, std::forward<T3>(a1), std::forward<T4>(a2)...);
}

template <typename T1, typename T2>
std::optional<T1> f629(const std::optional<T1> &a0, T2 &&a1)
  requires(std::is_copy_constructible_v<T1>) && (std::invocable<T2>)
{
  return a0.template or_else<T2>(std::forward<T2>(a1));
}

template <typename T1, typename T2>
std::optional<T1> f630(std::optional<T1> &&a0, T2 &&a1)
  requires(std::is_move_constructible_v<T1>) && (std::invocable<T2>)
{
  return std::move(a0).template or_else<T2>(std::forward<T2>(a1));
}
