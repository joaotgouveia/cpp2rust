#include <regex>
#include <utility>

template <typename T1, typename T2 = void>
const std::sub_match<T1> &f17(const std::sub_match<T1> &a0,
                              const std::sub_match<T1> &a1)
  requires((
      std::is_copy_assignable_v<const typename std::pair<T1, T1>::first_type> &&
      std::is_copy_assignable_v<const typename std::pair<T1, T1>::second_type>))
{
  return a0.operator=(a1);
}

template <typename T1, typename T2 = void>
const std::sub_match<T1> &f18(const std::sub_match<T1> &a0,
                              std::sub_match<T1> &&a1)
  requires(
      (std::is_assignable_v<const typename std::pair<T1, T1>::first_type &,
                            typename std::pair<T1, T1>::first_type> &&
       std::is_assignable_v<const typename std::pair<T1, T1>::second_type &,
                            typename std::pair<T1, T1>::second_type>))
{
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2, typename T3>
const std::sub_match<T1> &f19(const std::sub_match<T1> &a0,
                              const std::pair<T2, T3> &a1)
  requires((std::is_assignable_v<const typename std::pair<T1, T1>::first_type &,
                                 const T2 &> &&
            std::is_assignable_v<
                const typename std::pair<T1, T1>::second_type &,
                const T3 &>)) &&
          (!(std::is_same<T2, T1>::value && std::is_same<T3, T1>::value))
{
  return a0.template operator= <T2, T3>(a1);
}

template <typename T1, typename T2, typename T3>
const std::sub_match<T1> &f20(const std::sub_match<T1> &a0,
                              std::pair<T2, T3> &&a1)
  requires((std::is_assignable_v<const typename std::pair<T1, T1>::first_type &,
                                 T2> &&
            std::is_assignable_v<
                const typename std::pair<T1, T1>::second_type &, T3>)) &&
          (!(std::is_same<T2, T1>::value && std::is_same<T3, T1>::value))
{
  return a0.template operator= <T2, T3>(std::move(a1));
}

template <typename T1, typename T2>
std::sub_match<T1> &f21(std::sub_match<T1> &a0, T2 &&a1)
  requires((std::__different_from<T2, std::pair<T1, T1>> &&
            std::is_assignable_v<typename std::pair<T1, T1>::first_type &,
                                 decltype(std::get<0>(std::declval<T2>()))> &&
            std::is_assignable_v<typename std::pair<T1, T1>::second_type &,
                                 decltype(std::get<1>(std::declval<T2>()))>)) &&
          (!(std::is_same<
               T2, const std::__conditional_t<
                       std::is_copy_assignable<
                           typename std::pair<T1, T1>::first_type>::value &&
                           std::is_copy_assignable<
                               typename std::pair<T1, T1>::second_type>::value,
                       std::pair<T1, T1>, std::__nat> &>::value) &&
           !(std::is_same<
               T2, std::__conditional_t<
                       std::is_move_assignable<
                           typename std::pair<T1, T1>::first_type>::value &&
                           std::is_move_assignable<
                               typename std::pair<T1, T1>::second_type>::value,
                       std::pair<T1, T1>, std::__nat>>::value))
{
  return a0.template operator= <T2>(std::forward<T2>(a1));
}

template <typename T1, typename T2>
const std::sub_match<T1> &f22(const std::sub_match<T1> &a0, T2 &&a1)
  requires((std::__different_from<T2, std::pair<T1, T1>> &&
            std::is_assignable_v<const typename std::pair<T1, T1>::first_type &,
                                 decltype(std::get<0>(std::declval<T2>()))> &&
            std::is_assignable_v<
                const typename std::pair<T1, T1>::second_type &,
                decltype(std::get<1>(std::declval<T2>()))>)) &&
          (!(std::is_same<T2, const std::pair<T1, T1> &>::value) &&
           !(std::is_same<T2, std::pair<T1, T1>>::value))
{
  return a0.template operator= <T2>(std::forward<T2>(a1));
}
