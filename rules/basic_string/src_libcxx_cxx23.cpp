#include <initializer_list>
#include <string>
#include <string_view>
#include <utility>

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3>
f342(std::basic_string<T1, T2, T3> &&a0,
     typename std::basic_string<T1, T2, T3>::size_type a1, const T3 &a2) {
  return std::basic_string<T1, T2, T3>(std::move(a0), std::move(a1), a2);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3>
f343(std::basic_string<T1, T2, T3> &&a0,
     typename std::basic_string<T1, T2, T3>::size_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2, const T3 &a3) {
  return std::basic_string<T1, T2, T3>(std::move(a0), std::move(a1),
                                       std::move(a2), a3);
}

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3>
f344(std::basic_string<T1, T2, T3> &&a0,
     typename std::basic_string<T1, T2, T3>::size_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return std::move(a0).substr(std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<T1, T2, T3> f345(std::from_range_t a0, T4 &&a1, const T3 &a2)
  requires(std::_ContainerCompatibleRange<T4, T1>)
{
  return std::basic_string<T1, T2, T3>(std::move(a0), std::forward<T4>(a1), a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<T1, T2, T3> &f346(std::basic_string<T1, T2, T3> &a0, T4 &&a1)
  requires(std::_ContainerCompatibleRange<T4, T1>)
{
  return a0.template append_range<T4>(std::forward<T4>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<T1, T2, T3> &f347(std::basic_string<T1, T2, T3> &a0, T4 &&a1)
  requires(std::_ContainerCompatibleRange<T4, T1>)
{
  return a0.template assign_range<T4>(std::forward<T4>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::basic_string<T1, T2, T3>::iterator
f348(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1, T4 &&a2)
  requires(std::_ContainerCompatibleRange<T4, T1>)
{
  return a0.template insert_range<T4>(std::move(a1), std::forward<T4>(a2));
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<T1, T2, T3> &
f349(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1,
     typename std::basic_string<T1, T2, T3>::const_iterator a2, T4 &&a3)
  requires(std::_ContainerCompatibleRange<T4, T1>)
{
  return a0.template replace_with_range<T4>(std::move(a1), std::move(a2),
                                            std::forward<T4>(a3));
}
