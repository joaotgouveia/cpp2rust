#include <algorithm>
#include <initializer_list>
#include <map>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::__enable_if_t<
    std::is_constructible<std::pair<const T1, T2>, T5>::value,
    std::pair<typename std::_Rb_tree<
                  T1, std::pair<const T1, T2>,
                  std::_Select1st<std::pair<const T1, T2>>, T3,
                  typename __gnu_cxx::__alloc_traits<T4>::template rebind<
                      std::pair<const T1, T2>>::other>::iterator,
              bool>>
f94(std::map<T1, T2, T3, T4> &a0, T5 &&a1)
  requires(
      !(std::is_same<T5,
                     typename std::map<T1, T2, T3, T4>::node_type>::value) &&
      !(std::is_same<
          T5, const typename std::map<T1, T2, T3, T4>::value_type &>::value) &&
      !(std::is_same<T5,
                     typename std::map<T1, T2, T3, T4>::value_type>::value) &&
      !(std::is_same<T5, std::initializer_list<typename std::map<
                             T1, T2, T3, T4>::value_type>>::value))
{
  return a0.template insert<T5>(std::forward<T5>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::__enable_if_t<std::is_constructible<std::pair<const T1, T2>, T5>::value,
                   typename std::_Rb_tree<
                       T1, std::pair<const T1, T2>,
                       std::_Select1st<std::pair<const T1, T2>>, T3,
                       typename __gnu_cxx::__alloc_traits<T4>::template rebind<
                           std::pair<const T1, T2>>::other>::iterator>
f95(std::map<T1, T2, T3, T4> &a0,
    typename std::map<T1, T2, T3, T4>::const_iterator a1, T5 &&a2)
  requires(
      !(std::is_same<T5,
                     typename std::map<T1, T2, T3, T4>::node_type>::value) &&
      !(std::is_same<
          T5, const typename std::map<T1, T2, T3, T4>::value_type &>::value) &&
      !(std::is_same<T5, typename std::map<T1, T2, T3, T4>::value_type>::value))
{
  return a0.template insert<T5>(std::move(a1), std::forward<T5>(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f96(std::map<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(
      !(std::is_same<T5, typename std::map<T1, T2, T3, T4>::key_type>::value))
{
  return a0.template find<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f97(const std::map<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(
      !(std::is_same<T5, typename std::map<T1, T2, T3, T4>::key_type>::value))
{
  return a0.template find<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f98(const std::map<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(
      !(std::is_same<T5, typename std::map<T1, T2, T3, T4>::key_type>::value))
{
  return a0.template count<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f99(const std::map<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(
      !(std::is_same<T5, typename std::map<T1, T2, T3, T4>::key_type>::value))
{
  return a0.template contains<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f100(std::map<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(
      !(std::is_same<T5, typename std::map<T1, T2, T3, T4>::key_type>::value))
{
  return a0.template lower_bound<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f101(const std::map<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(
      !(std::is_same<T5, typename std::map<T1, T2, T3, T4>::key_type>::value))
{
  return a0.template lower_bound<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f102(std::map<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(
      !(std::is_same<T5, typename std::map<T1, T2, T3, T4>::key_type>::value))
{
  return a0.template upper_bound<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f103(const std::map<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(
      !(std::is_same<T5, typename std::map<T1, T2, T3, T4>::key_type>::value))
{
  return a0.template upper_bound<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f104(std::map<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(
      !(std::is_same<T5, typename std::map<T1, T2, T3, T4>::key_type>::value))
{
  return a0.template equal_range<T5>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
auto f105(const std::map<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(
      !(std::is_same<T5, typename std::map<T1, T2, T3, T4>::key_type>::value))
{
  return a0.template equal_range<T5>(a1);
}
