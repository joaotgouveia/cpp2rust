#include <algorithm>
#include <initializer_list>
#include <unordered_set>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_set<T1, T2, T3, T4>
f88(std::from_range_t a0, T5 &&a1,
    typename std::unordered_set<T1, T2, T3, T4>::size_type a2,
    const typename std::unordered_set<T1, T2, T3, T4>::hasher &a3,
    const typename std::unordered_set<T1, T2, T3, T4>::key_equal &a4,
    const typename std::unordered_set<T1, T2, T3, T4>::allocator_type &a5)
  requires(std::__detail::__container_compatible_range<T5, T1>)
{
  return std::unordered_set<T1, T2, T3, T4>(std::move(a0), std::forward<T5>(a1),
                                            std::move(a2), a3, a4, a5);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_set<T1, T2, T3, T4>
f89(std::from_range_t a0, T5 &&a1,
    const typename std::unordered_set<T1, T2, T3, T4>::allocator_type &a2)
  requires(std::__detail::__container_compatible_range<T5, T1>)
{
  return std::unordered_set<T1, T2, T3, T4>(std::move(a0), std::forward<T5>(a1),
                                            a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_set<T1, T2, T3, T4>
f90(std::from_range_t a0, T5 &&a1,
    typename std::unordered_set<T1, T2, T3, T4>::size_type a2,
    const typename std::unordered_set<T1, T2, T3, T4>::allocator_type &a3)
  requires(std::__detail::__container_compatible_range<T5, T1>)
{
  return std::unordered_set<T1, T2, T3, T4>(std::move(a0), std::forward<T5>(a1),
                                            std::move(a2), a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_set<T1, T2, T3, T4>
f91(std::from_range_t a0, T5 &&a1,
    typename std::unordered_set<T1, T2, T3, T4>::size_type a2,
    const typename std::unordered_set<T1, T2, T3, T4>::hasher &a3,
    const typename std::unordered_set<T1, T2, T3, T4>::allocator_type &a4)
  requires(std::__detail::__container_compatible_range<T5, T1>)
{
  return std::unordered_set<T1, T2, T3, T4>(std::move(a0), std::forward<T5>(a1),
                                            std::move(a2), a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f92(std::unordered_set<T1, T2, T3, T4> &a0, T5 &&a1)
  requires(std::__detail::__container_compatible_range<T5, T1>)
{
  return a0.template insert_range<T5>(std::forward<T5>(a1));
}
