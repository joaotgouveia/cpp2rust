#include <initializer_list>
#include <ranges>
#include <unordered_set>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multiset<T1, T2, T3, T4>
f126(std::from_range_t a0, T5 &&a1,
     typename std::unordered_multiset<T1, T2, T3, T4>::size_type a2,
     const T2 &a3, const T3 &a4, const T4 &a5)
  requires(std::_ContainerCompatibleRange<
           T5, typename std::unordered_multiset<T1, T2, T3, T4>::value_type>)
{
  return std::unordered_multiset<T1, T2, T3, T4>(
      std::move(a0), std::forward<T5>(a1), a2, a3, a4, a5);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multiset<T1, T2, T3, T4>
f127(std::from_range_t a0, T5 &&a1,
     typename std::unordered_multiset<T1, T2, T3, T4>::size_type a2,
     const T4 &a3)
  requires(std::_ContainerCompatibleRange<
           T5, typename std::unordered_multiset<T1, T2, T3, T4>::value_type>)
{
  return std::unordered_multiset<T1, T2, T3, T4>(std::move(a0),
                                                 std::forward<T5>(a1), a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_multiset<T1, T2, T3, T4>
f128(std::from_range_t a0, T5 &&a1,
     typename std::unordered_multiset<T1, T2, T3, T4>::size_type a2,
     const T2 &a3, const T4 &a4)
  requires(std::_ContainerCompatibleRange<
           T5, typename std::unordered_multiset<T1, T2, T3, T4>::value_type>)
{
  return std::unordered_multiset<T1, T2, T3, T4>(
      std::move(a0), std::forward<T5>(a1), a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f129(std::unordered_multiset<T1, T2, T3, T4> &a0, T5 &&a1)
  requires(std::_ContainerCompatibleRange<
           T5, typename std::unordered_multiset<T1, T2, T3, T4>::value_type>)
{
  return a0.template insert_range<T5>(std::forward<T5>(a1));
}
