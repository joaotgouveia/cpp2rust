#include <algorithm>
#include <ranges>
#include <utility>

template <typename T1, typename T2>
auto f51(std::ranges::lazy_split_view<T1, T2> &a0)
  requires(std::ranges::input_range<std::ranges::lazy_split_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::lazy_split_view<T1, T2>> &&
           std::same_as<
               std::ranges::lazy_split_view<T1, T2>,
               std::remove_cv_t<std::ranges::lazy_split_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to> &&
           (std::ranges::forward_range<T1> ||
            std::ranges::__detail::__tiny_range<T2>))
{
  return a0.cbegin();
}

template <typename T1, typename T2>
auto f52(const std::ranges::lazy_split_view<T1, T2> &a0)
  requires(std::ranges::input_range<
              const std::ranges::lazy_split_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::lazy_split_view<T1, T2>> &&
           std::same_as<
               std::ranges::lazy_split_view<T1, T2>,
               std::remove_cv_t<std::ranges::lazy_split_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to> &&
           (std::ranges::forward_range<T1> ||
            std::ranges::__detail::__tiny_range<T2>))
{
  return a0.cbegin();
}

template <typename T1, typename T2>
auto f53(std::ranges::lazy_split_view<T1, T2> &a0)
  requires(std::ranges::input_range<std::ranges::lazy_split_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::lazy_split_view<T1, T2>> &&
           std::same_as<
               std::ranges::lazy_split_view<T1, T2>,
               std::remove_cv_t<std::ranges::lazy_split_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to> &&
           (std::ranges::forward_range<T1> ||
            std::ranges::__detail::__tiny_range<T2>))
{
  return a0.cend();
}

template <typename T1, typename T2>
auto f54(const std::ranges::lazy_split_view<T1, T2> &a0)
  requires(std::ranges::input_range<
              const std::ranges::lazy_split_view<T1, T2>>) &&
          (std::is_class_v<std::ranges::lazy_split_view<T1, T2>> &&
           std::same_as<
               std::ranges::lazy_split_view<T1, T2>,
               std::remove_cv_t<std::ranges::lazy_split_view<T1, T2>>>) &&
          (std::ranges::view<T1> && std::ranges::view<T2> &&
           std::indirectly_comparable<std::ranges::iterator_t<T1>,
                                      std::ranges::iterator_t<T2>,
                                      std::ranges::equal_to> &&
           (std::ranges::forward_range<T1> ||
            std::ranges::__detail::__tiny_range<T2>))
{
  return a0.cend();
}

template <typename T1, typename T2, bool T3>
auto f55(typename std::ranges::lazy_split_view<T1, T2>::template _OuterIter<
         T3>::value_type &a0)
  requires(std::ranges::input_range<typename std::ranges::lazy_split_view<
               T1, T2>::template _OuterIter<T3>::value_type>) &&
          (std::is_class_v<typename std::ranges::lazy_split_view<
               T1, T2>::template _OuterIter<T3>::value_type> &&
           std::same_as<typename std::ranges::lazy_split_view<
                            T1, T2>::template _OuterIter<T3>::value_type,
                        std::remove_cv_t<typename std::ranges::lazy_split_view<
                            T1, T2>::template _OuterIter<T3>::value_type>>)
{
  return a0.cbegin();
}

template <typename T1, typename T2, bool T3>
auto f56(const typename std::ranges::lazy_split_view<
         T1, T2>::template _OuterIter<T3>::value_type &a0)
  requires(std::ranges::input_range<const typename std::ranges::lazy_split_view<
               T1, T2>::template _OuterIter<T3>::value_type>) &&
          (std::is_class_v<typename std::ranges::lazy_split_view<
               T1, T2>::template _OuterIter<T3>::value_type> &&
           std::same_as<typename std::ranges::lazy_split_view<
                            T1, T2>::template _OuterIter<T3>::value_type,
                        std::remove_cv_t<typename std::ranges::lazy_split_view<
                            T1, T2>::template _OuterIter<T3>::value_type>>)
{
  return a0.cbegin();
}

template <typename T1, typename T2, bool T3>
auto f57(typename std::ranges::lazy_split_view<T1, T2>::template _OuterIter<
         T3>::value_type &a0)
  requires(std::ranges::input_range<typename std::ranges::lazy_split_view<
               T1, T2>::template _OuterIter<T3>::value_type>) &&
          (std::is_class_v<typename std::ranges::lazy_split_view<
               T1, T2>::template _OuterIter<T3>::value_type> &&
           std::same_as<typename std::ranges::lazy_split_view<
                            T1, T2>::template _OuterIter<T3>::value_type,
                        std::remove_cv_t<typename std::ranges::lazy_split_view<
                            T1, T2>::template _OuterIter<T3>::value_type>>)
{
  return a0.cend();
}

template <typename T1, typename T2, bool T3>
auto f58(const typename std::ranges::lazy_split_view<
         T1, T2>::template _OuterIter<T3>::value_type &a0)
  requires(std::ranges::input_range<const typename std::ranges::lazy_split_view<
               T1, T2>::template _OuterIter<T3>::value_type>) &&
          (std::is_class_v<typename std::ranges::lazy_split_view<
               T1, T2>::template _OuterIter<T3>::value_type> &&
           std::same_as<typename std::ranges::lazy_split_view<
                            T1, T2>::template _OuterIter<T3>::value_type,
                        std::remove_cv_t<typename std::ranges::lazy_split_view<
                            T1, T2>::template _OuterIter<T3>::value_type>>)
{
  return a0.cend();
}
