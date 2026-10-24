#include <array>
#include <tuple>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
auto f81(const std::pair<T1, T2> &a0, const std::pair<T3, T4> &a1) {
  return std::operator<=> <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<std::__check_pair_construction<T1, T2>::
                                 template __is_pair_constructible<T3, T4>(),
                             int>
              T5 = 0>
std::pair<T1, T2> f82(T3 &&a0, T4 &&a1)
  requires(!(std::is_same<T3, const T1 &>::value &&
             std::is_same<T4, const T2 &>::value))
{
  return std::pair<T1, T2>(std::forward<T3>(a0), std::forward<T4>(a1));
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<
              std::__check_pair_construction<T1, T2>::
                  template __is_pair_constructible<const T3 &, const T4 &>(),
              int>
              T5 = 0>
std::pair<T1, T2> f83(const std::pair<T3, T4> &a0)
  requires(!(std::is_same<T3, T1>::value && std::is_same<T4, T2>::value))
{
  return std::pair<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<std::__check_pair_construction<T1, T2>::
                                 template __is_pair_constructible<T3, T4>(),
                             int>
              T5 = 0>
std::pair<T1, T2> f84(std::pair<T3, T4> &&a0)
  requires(!(std::is_same<T3, T1>::value && std::is_same<T4, T2>::value))
{
  return std::pair<T1, T2>(std::move(a0));
}
