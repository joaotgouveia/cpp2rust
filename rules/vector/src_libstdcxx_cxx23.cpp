#include <algorithm>
#include <utility>
#include <vector>

template <typename T1, typename T2>
std::vector<bool, T1> f128(std::from_range_t a0, T2 &&a1, const T1 &a2)
  requires(std::__detail::__container_compatible_range<T2, bool>)
{
  return std::vector<bool, T1>(std::move(a0), std::forward<T2>(a1), a2);
}

template <typename T1, typename T2>
void f129(std::vector<bool, T1> &a0, T2 &&a1)
  requires(std::__detail::__container_compatible_range<T2, bool>)
{
  return a0.template assign_range<T2>(std::forward<T2>(a1));
}

template <typename T1, typename T2>
std::_Bit_iterator f130(std::vector<bool, T1> &a0, std::_Bit_const_iterator a1,
                        T2 &&a2)
  requires(std::__detail::__container_compatible_range<T2, bool>)
{
  return a0.template insert_range<T2>(std::move(a1), std::forward<T2>(a2));
}

template <typename T1, typename T2>
void f131(std::vector<bool, T1> &a0, T2 &&a1)
  requires(std::__detail::__container_compatible_range<T2, bool>)
{
  return a0.template append_range<T2>(std::forward<T2>(a1));
}

template <typename T1, typename T2, typename T3>
std::vector<T1, T2> f132(std::from_range_t a0, T3 &&a1, const T2 &a2)
  requires(std::__detail::__container_compatible_range<T3, T1>)
{
  return std::vector<T1, T2>(std::move(a0), std::forward<T3>(a1), a2);
}

template <typename T1, typename T2, typename T3>
void f133(std::vector<T1, T2> &a0, T3 &&a1)
  requires(std::__detail::__container_compatible_range<T3, T1>)
{
  return a0.template assign_range<T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
typename std::vector<T1, T2>::iterator
f134(std::vector<T1, T2> &a0, typename std::vector<T1, T2>::const_iterator a1,
     T3 &&a2)
  requires(std::__detail::__container_compatible_range<T3, T1>)
{
  return a0.template insert_range<T3>(a1, std::forward<T3>(a2));
}

template <typename T1, typename T2, typename T3>
void f135(std::vector<T1, T2> &a0, T3 &&a1)
  requires(std::__detail::__container_compatible_range<T3, T1>)
{
  return a0.template append_range<T3>(std::forward<T3>(a1));
}
