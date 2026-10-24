#include <ranges>
#include <utility>

template <typename T1, typename... T2>
auto f1(T2 &&...a0)
  requires((!std::ranges::view<T1>))
{
  return std::ranges::to<T1, T2...>(std::forward<T2>(a0)...);
}

template <template <typename...> typename T1, typename... T2>
auto f2(T2 &&...a0) {
  return std::ranges::to<T1, T2...>(std::forward<T2>(a0)...);
}

template <typename T1, typename T2, typename... T3>
T1 f3(T2 &&a0, T3 &&...a1)
  requires((!std::ranges::view<T1>)) && (std::ranges::input_range<T2>)
{
  return std::ranges::to<T1, T2, T3...>(std::forward<T2>(a0),
                                        std::forward<T3>(a1)...);
}

template <template <typename...> typename T1, typename T2, typename... T3>
auto f4(T2 &&a0, T3 &&...a1)
  requires(std::ranges::input_range<T2>)
{
  return std::ranges::to<T1, T2, T3...>(std::forward<T2>(a0),
                                        std::forward<T3>(a1)...);
}
