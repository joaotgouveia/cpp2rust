#include <algorithm>
#include <initializer_list>
#include <map>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::multimap<T1, T2, T3, T4> f92(std::from_range_t a0, T5 &&a1, const T3 &a2,
                                  const T4 &a3)
  requires(std::__detail::__container_compatible_range<
           T5, typename std::multimap<T1, T2, T3, T4>::value_type>)
{
  return std::multimap<T1, T2, T3, T4>(std::move(a0), std::forward<T5>(a1), a2,
                                       a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::multimap<T1, T2, T3, T4> f93(std::from_range_t a0, T5 &&a1, const T4 &a2)
  requires(std::__detail::__container_compatible_range<
           T5, typename std::multimap<T1, T2, T3, T4>::value_type>)
{
  return std::multimap<T1, T2, T3, T4>(std::move(a0), std::forward<T5>(a1), a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f94(std::multimap<T1, T2, T3, T4> &a0, T5 &&a1)
  requires(std::__detail::__container_compatible_range<
           T5, typename std::multimap<T1, T2, T3, T4>::value_type>)
{
  return a0.template insert_range<T5>(std::forward<T5>(a1));
}
