#include <forward_list>
#include <initializer_list>
#include <ranges>
#include <utility>

template <typename T1, typename T2, typename T3>
std::forward_list<T1, T2> f101(std::from_range_t a0, T3 &&a1, const T2 &a2)
  requires(std::_ContainerCompatibleRange<T3, T1>)
{
  return std::forward_list<T1, T2>(std::move(a0), std::forward<T3>(a1), a2);
}

template <typename T1, typename T2, typename T3>
void f102(std::forward_list<T1, T2> &a0, T3 &&a1)
  requires(std::_ContainerCompatibleRange<T3, T1>)
{
  return a0.template assign_range<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
void f103(std::forward_list<T1, T2> &a0, T3 &&a1)
  requires(std::_ContainerCompatibleRange<T3, T1>)
{
  return a0.template prepend_range<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
typename std::forward_list<T1, T2>::iterator
f104(std::forward_list<T1, T2> &a0,
     typename std::forward_list<T1, T2>::const_iterator a1, T3 &&a2)
  requires(std::_ContainerCompatibleRange<T3, T1>)
{
  return a0.template insert_range_after<T3>(a1, std::forward<T3>(a2));
}
