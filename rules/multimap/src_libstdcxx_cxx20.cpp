#include <algorithm>
#include <initializer_list>
#include <map>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::__enable_if_t<std::is_constructible<std::pair<const T1, T2>, T5>::value,
                   typename std::_Rb_tree<
                       T1, std::pair<const T1, T2>,
                       std::_Select1st<std::pair<const T1, T2>>, T3,
                       typename __gnu_cxx::__alloc_traits<T4>::template rebind<
                           std::pair<const T1, T2>>::other>::iterator>
f80(std::multimap<T1, T2, T3, T4> &a0, T5 &&a1)
  requires(
      !(std::is_same<T5, const typename std::multimap<
                             T1, T2, T3, T4>::value_type &>::value) &&
      !(std::is_same<
          T5, typename std::multimap<T1, T2, T3, T4>::value_type>::value) &&
      !(std::is_same<T5, std::initializer_list<typename std::multimap<
                             T1, T2, T3, T4>::value_type>>::value) &&
      !(std::is_same<T5,
                     typename std::multimap<T1, T2, T3, T4>::node_type>::value))
{
  return a0.template insert<T5>(std::forward<T5>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::__enable_if_t<std::is_constructible<std::pair<const T1, T2>, T5 &&>::value,
                   typename std::_Rb_tree<
                       T1, std::pair<const T1, T2>,
                       std::_Select1st<std::pair<const T1, T2>>, T3,
                       typename __gnu_cxx::__alloc_traits<T4>::template rebind<
                           std::pair<const T1, T2>>::other>::iterator>
f81(std::multimap<T1, T2, T3, T4> &a0,
    typename std::multimap<T1, T2, T3, T4>::const_iterator a1, T5 &&a2)
  requires(
      !(std::is_same<T5, const typename std::multimap<
                             T1, T2, T3, T4>::value_type &>::value) &&
      !(std::is_same<
          T5, typename std::multimap<T1, T2, T3, T4>::value_type>::value) &&
      !(std::is_same<T5,
                     typename std::multimap<T1, T2, T3, T4>::node_type>::value))
{
  return a0.template insert<T5>(std::move(a1), std::forward<T5>(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f82(std::multimap<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(!(std::is_same<
             T5, typename std::multimap<T1, T2, T3, T4>::key_type>::value))
{
  return a0.template find<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f83(const std::multimap<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(!(std::is_same<
             T5, typename std::multimap<T1, T2, T3, T4>::key_type>::value))
{
  return a0.template find<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f84(const std::multimap<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(!(std::is_same<
             T5, typename std::multimap<T1, T2, T3, T4>::key_type>::value))
{
  return a0.template count<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f85(const std::multimap<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(!(std::is_same<
             T5, typename std::multimap<T1, T2, T3, T4>::key_type>::value))
{
  return a0.template contains<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f86(std::multimap<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(!(std::is_same<
             T5, typename std::multimap<T1, T2, T3, T4>::key_type>::value))
{
  return a0.template lower_bound<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f87(const std::multimap<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(!(std::is_same<
             T5, typename std::multimap<T1, T2, T3, T4>::key_type>::value))
{
  return a0.template lower_bound<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f88(std::multimap<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(!(std::is_same<
             T5, typename std::multimap<T1, T2, T3, T4>::key_type>::value))
{
  return a0.template upper_bound<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f89(const std::multimap<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(!(std::is_same<
             T5, typename std::multimap<T1, T2, T3, T4>::key_type>::value))
{
  return a0.template upper_bound<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f90(std::multimap<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(!(std::is_same<
             T5, typename std::multimap<T1, T2, T3, T4>::key_type>::value))
{
  return a0.template equal_range<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f91(const std::multimap<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(!(std::is_same<
             T5, typename std::multimap<T1, T2, T3, T4>::key_type>::value))
{
  return a0.template equal_range<T5>(a1);
}
