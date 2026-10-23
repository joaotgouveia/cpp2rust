#include <ranges>
#include <utility>

template <typename T1>
std::ranges::owning_view<T1> f31()
  requires(std::default_initializable<T1>) &&
          (std::movable<T1> &&
           (!std::ranges::__is_std_initializer_list<std::remove_cvref_t<T1>>))
{
  return std::ranges::owning_view<T1>();
}

template <typename T1>
std::ranges::owning_view<T1> f32(T1 &&a0)
  requires(std::movable<T1> &&
           (!std::ranges::__is_std_initializer_list<std::remove_cvref_t<T1>>))
{
  return std::ranges::owning_view<T1>(std::move(a0));
}

template <typename T1>
std::ranges::owning_view<T1> f33(std::ranges::owning_view<T1> &&a0)
  requires(std::movable<T1> &&
           (!std::ranges::__is_std_initializer_list<std::remove_cvref_t<T1>>))
{
  return std::ranges::owning_view<T1>(std::move(a0));
}

template <typename T1>
std::ranges::owning_view<T1> &f34(std::ranges::owning_view<T1> &a0,
                                  std::ranges::owning_view<T1> &&a1)
  requires(std::movable<T1> &&
           (!std::ranges::__is_std_initializer_list<std::remove_cvref_t<T1>>))
{
  return a0.operator=(std::move(a1));
}

template <typename T1>
T1 &f35(std::ranges::owning_view<T1> &a0)
  requires(std::movable<T1> &&
           (!std::ranges::__is_std_initializer_list<std::remove_cvref_t<T1>>))
{
  return a0.base();
}

template <typename T1>
const T1 &f36(const std::ranges::owning_view<T1> &a0)
  requires(std::movable<T1> &&
           (!std::ranges::__is_std_initializer_list<std::remove_cvref_t<T1>>))
{
  return a0.base();
}

template <typename T1>
T1 &&f37(std::ranges::owning_view<T1> &&a0)
  requires(std::movable<T1> &&
           (!std::ranges::__is_std_initializer_list<std::remove_cvref_t<T1>>))
{
  return std::move(a0).base();
}

template <typename T1>
const T1 &&f38(const std::ranges::owning_view<T1> &&a0)
  requires(std::movable<T1> &&
           (!std::ranges::__is_std_initializer_list<std::remove_cvref_t<T1>>))
{
  return std::move(a0).base();
}

template <typename T1>
std::ranges::iterator_t<T1> f39(std::ranges::owning_view<T1> &a0)
  requires(std::movable<T1> &&
           (!std::ranges::__is_std_initializer_list<std::remove_cvref_t<T1>>))
{
  return a0.begin();
}

template <typename T1>
std::ranges::sentinel_t<T1> f40(std::ranges::owning_view<T1> &a0)
  requires(std::movable<T1> &&
           (!std::ranges::__is_std_initializer_list<std::remove_cvref_t<T1>>))
{
  return a0.end();
}

template <typename T1>
auto f41(const std::ranges::owning_view<T1> &a0)
  requires(std::ranges::range<const T1>) &&
          (std::movable<T1> &&
           (!std::ranges::__is_std_initializer_list<std::remove_cvref_t<T1>>))
{
  return a0.begin();
}

template <typename T1>
auto f42(const std::ranges::owning_view<T1> &a0)
  requires(std::ranges::range<const T1>) &&
          (std::movable<T1> &&
           (!std::ranges::__is_std_initializer_list<std::remove_cvref_t<T1>>))
{
  return a0.end();
}

template <typename T1>
auto f43(std::ranges::owning_view<T1> &a0)
  requires(std::ranges::sized_range<T1>) &&
          (std::movable<T1> &&
           (!std::ranges::__is_std_initializer_list<std::remove_cvref_t<T1>>))
{
  return a0.size();
}

template <typename T1>
auto f44(const std::ranges::owning_view<T1> &a0)
  requires(std::ranges::sized_range<const T1>) &&
          (std::movable<T1> &&
           (!std::ranges::__is_std_initializer_list<std::remove_cvref_t<T1>>))
{
  return a0.size();
}

template <typename T1>
auto f45(std::ranges::owning_view<T1> &a0)
  requires(std::ranges::contiguous_range<T1>) &&
          (std::movable<T1> &&
           (!std::ranges::__is_std_initializer_list<std::remove_cvref_t<T1>>))
{
  return a0.data();
}

template <typename T1>
auto f46(const std::ranges::owning_view<T1> &a0)
  requires(std::ranges::contiguous_range<const T1>) &&
          (std::movable<T1> &&
           (!std::ranges::__is_std_initializer_list<std::remove_cvref_t<T1>>))
{
  return a0.data();
}

template <typename T1, typename T2 = std::ranges::owning_view<T1>>
bool f47(std::ranges::owning_view<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2 = std::ranges::owning_view<T1>>
bool f48(const std::ranges::owning_view<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1, typename T2 = std::ranges::owning_view<T1>>
decltype(auto) f49(std::ranges::owning_view<T1> &a0)
  requires(std::ranges::forward_range<std::ranges::owning_view<T1>>) &&
          (std::is_class_v<std::ranges::owning_view<T1>> &&
           std::same_as<std::ranges::owning_view<T1>,
                        std::remove_cv_t<std::ranges::owning_view<T1>>>)
{
  return a0.front();
}

template <typename T1, typename T2 = std::ranges::owning_view<T1>>
decltype(auto) f50(const std::ranges::owning_view<T1> &a0)
  requires(std::ranges::forward_range<std::ranges::owning_view<T1> const>) &&
          (std::is_class_v<std::ranges::owning_view<T1>> &&
           std::same_as<std::ranges::owning_view<T1>,
                        std::remove_cv_t<std::ranges::owning_view<T1>>>)
{
  return a0.front();
}

template <typename T1, typename T2 = std::ranges::owning_view<T1>>
decltype(auto) f51(std::ranges::owning_view<T1> &a0)
  requires(std::ranges::bidirectional_range<std::ranges::owning_view<T1>> &&
           std::ranges::common_range<std::ranges::owning_view<T1>>) &&
          (std::is_class_v<std::ranges::owning_view<T1>> &&
           std::same_as<std::ranges::owning_view<T1>,
                        std::remove_cv_t<std::ranges::owning_view<T1>>>)
{
  return a0.back();
}

template <typename T1, typename T2 = std::ranges::owning_view<T1>>
decltype(auto) f52(const std::ranges::owning_view<T1> &a0)
  requires(std::ranges::bidirectional_range<
               std::ranges::owning_view<T1> const> &&
           std::ranges::common_range<std::ranges::owning_view<T1> const>) &&
          (std::is_class_v<std::ranges::owning_view<T1>> &&
           std::same_as<std::ranges::owning_view<T1>,
                        std::remove_cv_t<std::ranges::owning_view<T1>>>)
{
  return a0.back();
}
