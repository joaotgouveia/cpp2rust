#include <ranges>
#include <utility>

template <typename T1> using t1 = std::ranges::istream_view<T1>;

template <typename T1> using t2 = std::ranges::keys_view<T1>;

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

template <typename T1> using t3 = std::ranges::values_view<T1>;

template <typename T1, typename... T2>
using t4 = std::ranges::views::__adaptor::_Partial<T1, T2...>;

template <typename T1, typename... T2, typename... T3>
std::ranges::views::__adaptor::_Partial<T1, T2...> f5(int a0, T3 &&...a1) {
  return std::ranges::views::__adaptor::_Partial<T1, T2...>(
      a0, std::forward<T3>(a1)...);
}

template <typename T1, typename... T2, typename T3, typename T4>
auto f6(std::ranges::views::__adaptor::_Partial<T1, T2...> &a0, T3 &&a1,
        T4 &&a2) {
  return a0.template operator()<T3, T4>(std::forward<T3>(a1),
                                        std::forward<T4>(a2));
}

template <typename T1> using t5 = std::ranges::views::all_t<T1>;

template <typename T1> std::ranges::empty_view<T1> f7() {
  return std::ranges::views::empty<T1>;
}

template <typename T1> using t6 = std::ranges::wistream_view<T1>;
