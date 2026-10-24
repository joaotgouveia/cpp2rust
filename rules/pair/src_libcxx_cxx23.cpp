#include <tuple>
#include <utility>

template <typename T1, typename T2>
void f79(const std::pair<T1, T2> &a0, const std::pair<T1, T2> &a1)
  requires((std::__is_swappable_v<const T1> && std::__is_swappable_v<const T2>))
{
  return std::swap<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
void f80(const std::pair<T1, T2> &a0, const std::pair<T1, T2> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<std::__check_pair_construction<T1, T2>::
                                 template __is_pair_constructible<T3 &, T4 &>(),
                             int>
              T5 = 0>
std::pair<T1, T2> f81(std::pair<T3, T4> &a0) {
  return std::pair<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<
              std::__check_pair_construction<T1, T2>::
                  template __is_pair_constructible<const T3 &&, const T4 &&>(),
              int>
              T5 = 0>
std::pair<T1, T2> f82(const std::pair<T3, T4> &&a0) {
  return std::pair<T1, T2>(std::move(a0));
}

template <typename T1, typename T2, typename T3>
std::pair<T1, T2> f83(T3 &&a0)
  requires((std::is_constructible_v<
                typename std::pair<T1, T2>::first_type,
                decltype(std::get<0>(std::declval<T3 &&>()))> &&
            std::is_constructible_v<
                typename std::pair<T1, T2>::second_type,
                decltype(std::get<1>(std::declval<T3 &&>()))>)) &&
          (std::__pair_like_no_subrange<T3>) &&
          (!(std::is_same<T3, const std::pair<T1, T2> &>::value) &&
           !(std::is_same<T3, std::pair<T1, T2>>::value))
{
  return std::pair<T1, T2>(std::forward<T3>(a0));
}

template <typename T1, typename T2, typename T3 = void>
const std::pair<T1, T2> &f84(const std::pair<T1, T2> &a0,
                             const std::pair<T1, T2> &a1)
  requires((
      std::is_copy_assignable_v<const typename std::pair<T1, T2>::first_type> &&
      std::is_copy_assignable_v<const typename std::pair<T1, T2>::second_type>))
{
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3 = void>
const std::pair<T1, T2> &f85(const std::pair<T1, T2> &a0,
                             std::pair<T1, T2> &&a1)
  requires(
      (std::is_assignable_v<const typename std::pair<T1, T2>::first_type &,
                            typename std::pair<T1, T2>::first_type> &&
       std::is_assignable_v<const typename std::pair<T1, T2>::second_type &,
                            typename std::pair<T1, T2>::second_type>))
{
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
const std::pair<T1, T2> &f86(const std::pair<T1, T2> &a0,
                             const std::pair<T3, T4> &a1)
  requires((std::is_assignable_v<const typename std::pair<T1, T2>::first_type &,
                                 const T3 &> &&
            std::is_assignable_v<
                const typename std::pair<T1, T2>::second_type &,
                const T4 &>)) &&
          (!(std::is_same<T3, T1>::value && std::is_same<T4, T2>::value))
{
  return a0.template operator= <T3, T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
const std::pair<T1, T2> &f87(const std::pair<T1, T2> &a0,
                             std::pair<T3, T4> &&a1)
  requires((std::is_assignable_v<const typename std::pair<T1, T2>::first_type &,
                                 T3> &&
            std::is_assignable_v<
                const typename std::pair<T1, T2>::second_type &, T4>)) &&
          (!(std::is_same<T3, T1>::value && std::is_same<T4, T2>::value))
{
  return a0.template operator= <T3, T4>(std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::pair<T1, T2> &f88(std::pair<T1, T2> &a0, T3 &&a1)
  requires((std::__different_from<T3, std::pair<T1, T2>> &&
            std::is_assignable_v<typename std::pair<T1, T2>::first_type &,
                                 decltype(std::get<0>(std::declval<T3>()))> &&
            std::is_assignable_v<typename std::pair<T1, T2>::second_type &,
                                 decltype(std::get<1>(std::declval<T3>()))>)) &&
          (std::__pair_like_no_subrange<T3>) &&
          (!(std::is_same<
               T3, const std::__conditional_t<
                       std::is_copy_assignable<
                           typename std::pair<T1, T2>::first_type>::value &&
                           std::is_copy_assignable<
                               typename std::pair<T1, T2>::second_type>::value,
                       std::pair<T1, T2>, std::__nat> &>::value) &&
           !(std::is_same<
               T3, std::__conditional_t<
                       std::is_move_assignable<
                           typename std::pair<T1, T2>::first_type>::value &&
                           std::is_move_assignable<
                               typename std::pair<T1, T2>::second_type>::value,
                       std::pair<T1, T2>, std::__nat>>::value))
{
  return a0.template operator= <T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
const std::pair<T1, T2> &f89(const std::pair<T1, T2> &a0, T3 &&a1)
  requires((std::__different_from<T3, std::pair<T1, T2>> &&
            std::is_assignable_v<const typename std::pair<T1, T2>::first_type &,
                                 decltype(std::get<0>(std::declval<T3>()))> &&
            std::is_assignable_v<
                const typename std::pair<T1, T2>::second_type &,
                decltype(std::get<1>(std::declval<T3>()))>)) &&
          (std::__pair_like_no_subrange<T3>) &&
          (!(std::is_same<T3, const std::pair<T1, T2> &>::value) &&
           !(std::is_same<T3, std::pair<T1, T2>>::value))
{
  return a0.template operator= <T3>(std::forward<T3>(a1));
}
