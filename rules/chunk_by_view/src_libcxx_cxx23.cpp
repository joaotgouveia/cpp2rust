#include <ranges>
#include <utility>

template <typename T1, typename T2>
std::ranges::chunk_by_view<T1, T2> f28()
  requires(std::default_initializable<T1> && std::default_initializable<T2>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return std::ranges::chunk_by_view<T1, T2>();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::chunk_by_view<T1, T2>>
bool f29(std::ranges::chunk_by_view<T1, T2> &a0)
  requires(std::ranges::sized_range<std::ranges::chunk_by_view<T1, T2>> ||
           std::ranges::forward_range<std::ranges::chunk_by_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::chunk_by_view<T1, T2>> &&
           std::same_as<
               std::ranges::chunk_by_view<T1, T2>,
               std::remove_cv_t<std::ranges::chunk_by_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.empty();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::chunk_by_view<T1, T2>>
bool f30(const std::ranges::chunk_by_view<T1, T2> &a0)
  requires(std::ranges::sized_range<std::ranges::chunk_by_view<T1, T2> const> ||
           std::ranges::forward_range<
               std::ranges::chunk_by_view<T1, T2> const>) &&
          (std::is_class_v<std::ranges::chunk_by_view<T1, T2>> &&
           std::same_as<
               std::ranges::chunk_by_view<T1, T2>,
               std::remove_cv_t<std::ranges::chunk_by_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.empty();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::chunk_by_view<T1, T2>>
bool f31(std::ranges::chunk_by_view<T1, T2> &a0)
  requires(std::ranges::view<T1> && std::is_object_v<T2>)
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2,
          typename T3 = std::ranges::chunk_by_view<T1, T2>>
bool f32(const std::ranges::chunk_by_view<T1, T2> &a0)
  requires(std::ranges::view<T1> && std::is_object_v<T2>)
{
  return static_cast<bool>(a0);
}

template <typename T1, typename T2,
          typename T3 = std::ranges::chunk_by_view<T1, T2>>
auto f33(std::ranges::chunk_by_view<T1, T2> &a0)
  requires(std::contiguous_iterator<
              std::ranges::iterator_t<std::ranges::chunk_by_view<T1, T2>>>) &&
          (std::is_class_v<std::ranges::chunk_by_view<T1, T2>> &&
           std::same_as<
               std::ranges::chunk_by_view<T1, T2>,
               std::remove_cv_t<std::ranges::chunk_by_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.data();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::chunk_by_view<T1, T2>>
auto f34(const std::ranges::chunk_by_view<T1, T2> &a0)
  requires(std::ranges::range<std::ranges::chunk_by_view<T1, T2> const> &&
           std::contiguous_iterator<std::ranges::iterator_t<
               std::ranges::chunk_by_view<T1, T2> const>>) &&
          (std::is_class_v<std::ranges::chunk_by_view<T1, T2>> &&
           std::same_as<
               std::ranges::chunk_by_view<T1, T2>,
               std::remove_cv_t<std::ranges::chunk_by_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.data();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::chunk_by_view<T1, T2>>
auto f35(std::ranges::chunk_by_view<T1, T2> &a0)
  requires(std::ranges::forward_range<std::ranges::chunk_by_view<T1, T2>> &&
           std::sized_sentinel_for<
               std::ranges::sentinel_t<std::ranges::chunk_by_view<T1, T2>>,
               std::ranges::iterator_t<std::ranges::chunk_by_view<T1, T2>>>) &&
          (std::is_class_v<std::ranges::chunk_by_view<T1, T2>> &&
           std::same_as<
               std::ranges::chunk_by_view<T1, T2>,
               std::remove_cv_t<std::ranges::chunk_by_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.size();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::chunk_by_view<T1, T2>>
auto f36(const std::ranges::chunk_by_view<T1, T2> &a0)
  requires(std::ranges::forward_range<
               std::ranges::chunk_by_view<T1, T2> const> &&
           std::sized_sentinel_for<
               std::ranges::sentinel_t<
                   std::ranges::chunk_by_view<T1, T2> const>,
               std::ranges::iterator_t<
                   std::ranges::chunk_by_view<T1, T2> const>>) &&
          (std::is_class_v<std::ranges::chunk_by_view<T1, T2>> &&
           std::same_as<
               std::ranges::chunk_by_view<T1, T2>,
               std::remove_cv_t<std::ranges::chunk_by_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.size();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::chunk_by_view<T1, T2>>
decltype(auto) f37(std::ranges::chunk_by_view<T1, T2> &a0)
  requires(std::ranges::forward_range<std::ranges::chunk_by_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::chunk_by_view<T1, T2>> &&
           std::same_as<
               std::ranges::chunk_by_view<T1, T2>,
               std::remove_cv_t<std::ranges::chunk_by_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.front();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::chunk_by_view<T1, T2>>
decltype(auto) f38(const std::ranges::chunk_by_view<T1, T2> &a0)
  requires(std::ranges::forward_range<
              std::ranges::chunk_by_view<T1, T2> const>) &&
          (std::is_class_v<std::ranges::chunk_by_view<T1, T2>> &&
           std::same_as<
               std::ranges::chunk_by_view<T1, T2>,
               std::remove_cv_t<std::ranges::chunk_by_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.front();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::chunk_by_view<T1, T2>>
decltype(auto) f39(std::ranges::chunk_by_view<T1, T2> &a0)
  requires(std::ranges::bidirectional_range<
               std::ranges::chunk_by_view<T1, T2>> &&
           std::ranges::common_range<std::ranges::chunk_by_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::chunk_by_view<T1, T2>> &&
           std::same_as<
               std::ranges::chunk_by_view<T1, T2>,
               std::remove_cv_t<std::ranges::chunk_by_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.back();
}

template <typename T1, typename T2,
          typename T3 = std::ranges::chunk_by_view<T1, T2>>
decltype(auto) f40(const std::ranges::chunk_by_view<T1, T2> &a0)
  requires(std::ranges::bidirectional_range<
               std::ranges::chunk_by_view<T1, T2> const> &&
           std::ranges::common_range<
               std::ranges::chunk_by_view<T1, T2> const>) &&
          (std::is_class_v<std::ranges::chunk_by_view<T1, T2>> &&
           std::same_as<
               std::ranges::chunk_by_view<T1, T2>,
               std::remove_cv_t<std::ranges::chunk_by_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::is_object_v<T2>)
{
  return a0.back();
}
