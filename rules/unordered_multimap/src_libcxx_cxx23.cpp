#include <initializer_list>
#include <ranges>
#include <unordered_map>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::unordered_multimap<T1, T2, T3, T4, T5>
f138(std::from_range_t a0, T6 &&a1,
     typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a2,
     const T3 &a3, const T4 &a4, const T5 &a5)
  requires(
      std::_ContainerCompatibleRange<
          T6, typename std::unordered_multimap<T1, T2, T3, T4, T5>::value_type>)
{
  return std::unordered_multimap<T1, T2, T3, T4, T5>(
      std::move(a0), std::forward<T6>(a1),
      std::forward<
          typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type>(a2),
      a3, a4, a5);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::unordered_multimap<T1, T2, T3, T4, T5>
f139(std::from_range_t a0, T6 &&a1,
     typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a2,
     const T5 &a3)
  requires(
      std::_ContainerCompatibleRange<
          T6, typename std::unordered_multimap<T1, T2, T3, T4, T5>::value_type>)
{
  return std::unordered_multimap<T1, T2, T3, T4, T5>(
      std::move(a0), std::forward<T6>(a1),
      std::forward<
          typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type>(a2),
      a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::unordered_multimap<T1, T2, T3, T4, T5>
f140(std::from_range_t a0, T6 &&a1,
     typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type a2,
     const T3 &a3, const T5 &a4)
  requires(
      std::_ContainerCompatibleRange<
          T6, typename std::unordered_multimap<T1, T2, T3, T4, T5>::value_type>)
{
  return std::unordered_multimap<T1, T2, T3, T4, T5>(
      std::move(a0), std::forward<T6>(a1),
      std::forward<
          typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type>(a2),
      a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
void f141(std::unordered_multimap<T1, T2, T3, T4, T5> &a0, T6 &&a1)
  requires(
      std::_ContainerCompatibleRange<
          T6, typename std::unordered_multimap<T1, T2, T3, T4, T5>::value_type>)
{
  return a0.template insert_range<T6>(std::forward<T6>(a1));
}
