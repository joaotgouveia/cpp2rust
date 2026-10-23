#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1> using t1 = std::ranges::owning_view<T1>;

template <typename T1> bool f1(std::ranges::owning_view<T1> &a0) {
  return a0.empty();
}

template <typename T1> bool f2(const std::ranges::owning_view<T1> &a0) {
  return a0.empty();
}

template <typename T1, typename T2>
decltype(auto) f3(std::ranges::owning_view<T1> &a0,
                  std::ranges::range_difference_t<T2> a1)
  requires(std::is_class_v<std::ranges::owning_view<T1>> &&
           std::same_as<std::ranges::owning_view<T1>,
                        std::remove_cv_t<std::ranges::owning_view<T1>>>)
{
  return a0.template operator[]<T2>(a1);
}

template <typename T1, typename T2>
decltype(auto) f4(const std::ranges::owning_view<T1> &a0,
                  std::ranges::range_difference_t<T2> a1)
  requires(std::is_class_v<std::ranges::owning_view<T1>> &&
           std::same_as<std::ranges::owning_view<T1>,
                        std::remove_cv_t<std::ranges::owning_view<T1>>>)
{
  return a0.template operator[]<T2>(a1);
}
