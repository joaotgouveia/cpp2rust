#include <algorithm>
#include <utility>

template <typename T1, typename T2>
void f24(const std::pair<T1, T2> &a0, const std::pair<T1, T2> &a1)
  requires(std::is_swappable_v<const T1> && std::is_swappable_v<const T2>)
{
  return a0.swap(a1);
}

template <typename T1, typename T2>
const std::pair<T1, T2> &f25(const std::pair<T1, T2> &a0,
                             const std::pair<T1, T2> &a1)
  requires((std::pair<T1, T2>::template _S_const_assignable<
            const typename std::pair<T1, T2>::first_type &,
            const typename std::pair<T1, T2>::second_type &>()))
{
  return a0.operator=(a1);
}

template <typename T1, typename T2>
const std::pair<T1, T2> &f26(const std::pair<T1, T2> &a0,
                             std::pair<T1, T2> &&a1)
  requires((std::pair<T1, T2>::template _S_const_assignable<
            typename std::pair<T1, T2>::first_type,
            typename std::pair<T1, T2>::second_type>()))
{
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<T1, T2> f27(std::pair<T3, T4> &a0)
  requires((std::pair<T1, T2>::template _S_constructible<T3 &, T4 &>()) &&
           (!std::pair<T1, T2>::template _S_dangles<T3 &, T4 &>()))
{
  return std::pair<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<T1, T2> f28(const std::pair<T3, T4> &&a0)
  requires(
      (std::pair<T1, T2>::template _S_constructible<const T3, const T4>()) &&
      (!std::pair<T1, T2>::template _S_dangles<const T3, const T4>()))
{
  return std::pair<T1, T2>(std::move(a0));
}

template <typename T1, typename T2, typename T3>
std::pair<T1, T2> f29(T3 &&a0)
  requires((std::pair<T1,
                      T2>::template _S_constructible_from_pair_like<T3>()) &&
           (!std::pair<T1, T2>::template _S_dangles_from_pair_like<T3>())) &&
          (!(std::is_same<T3, const std::pair<T1, T2> &>::value) &&
           !(std::is_same<T3, std::pair<T1, T2>>::value))
{
  return std::pair<T1, T2>(std::forward<T3>(a0));
}

template <typename T1, typename T2, typename T3, typename T4>
const std::pair<T1, T2> &f30(const std::pair<T1, T2> &a0,
                             const std::pair<T3, T4> &a1)
  requires((std::pair<T1, T2>::template _S_const_assignable<const T3 &,
                                                            const T4 &>())) &&
          (!(std::is_same<T3, T1>::value && std::is_same<T4, T2>::value))
{
  return a0.template operator= <T3, T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
const std::pair<T1, T2> &f31(const std::pair<T1, T2> &a0,
                             std::pair<T3, T4> &&a1)
  requires((std::pair<T1, T2>::template _S_const_assignable<T3, T4>())) &&
          (!(std::is_same<T3, T1>::value && std::is_same<T4, T2>::value))
{
  return a0.template operator= <T3, T4>(std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::pair<T1, T2> &f32(std::pair<T1, T2> &a0, T3 &&a1)
  requires((std::pair<T1, T2>::template _S_assignable_from_tuple_like<T3>())) &&
          (!(std::is_same<T3, const std::pair<T1, T2> &>::value) &&
           !(std::is_same<T3, std::pair<T1, T2>>::value))
{
  return a0.template operator= <T3>(std::forward<T3>(a1));
}

template <typename T1, typename T2, typename T3>
const std::pair<T1, T2> &f33(const std::pair<T1, T2> &a0, T3 &&a1)
  requires((std::pair<T1, T2>::template _S_const_assignable_from_tuple_like<
              T3>())) &&
          (!(std::is_same<T3, const std::pair<T1, T2> &>::value) &&
           !(std::is_same<T3, std::pair<T1, T2>>::value))
{
  return a0.template operator= <T3>(std::forward<T3>(a1));
}
