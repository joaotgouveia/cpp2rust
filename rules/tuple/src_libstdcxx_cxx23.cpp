#include <algorithm>
#include <tuple>
#include <utility>

template <typename... T1>
void f95(const std::tuple<T1...> &a0, const std::tuple<T1...> &a1)
  requires((std::is_swappable_v<const T1> && ...))
{
  return std::swap<T1...>(a0, a1);
}

template <typename... T1>
const std::tuple<T1...> &f96(const std::tuple<T1...> &a0,
                             const std::tuple<T1...> &a1)
  requires((std::tuple<T1...>::template __const_assignable<const T1 &...>()))
{
  return a0.operator=(a1);
}

template <typename... T1>
const std::tuple<T1...> &f97(const std::tuple<T1...> &a0,
                             std::tuple<T1...> &&a1)
  requires((std::tuple<T1...>::template __const_assignable<T1...>()))
{
  return a0.operator=(std::move(a1));
}

template <typename... T1>
void f98(const std::tuple<T1...> &a0, const std::tuple<T1...> &a1)
  requires((std::is_swappable_v<const T1> && ...))
{
  return a0.swap(a1);
}

template <typename... T1, typename... T2>
std::tuple<T1...> f99(std::tuple<T2...> &a0)
  requires(
      (std::tuple<T1...>::template __constructible<T2 &...>()) &&
      (!std::tuple<T1...>::template __use_other_ctor<std::tuple<T2...> &>()) &&
      (!std::tuple<T1...>::template __dangles<T2 &...>()))
{
  return std::tuple<T1...>(a0);
}

template <typename... T1, typename... T2>
std::tuple<T1...> f100(const std::tuple<T2...> &&a0)
  requires((std::tuple<T1...>::template __constructible<const T2...>()) &&
           (!std::tuple<T1...>::template __use_other_ctor<
               const std::tuple<T2...>>()) &&
           (!std::tuple<T1...>::template __dangles<const T2...>()))
{
  return std::tuple<T1...>(std::move(a0));
}

template <typename... T1, typename T2, typename T3>
std::tuple<T1...> f101(std::pair<T2, T3> &a0)
  requires((sizeof...(T1) == 2) &&
           (std::tuple<T1...>::template __constructible<T2 &, T3 &>()) &&
           (!std::tuple<T1...>::template __dangles<T2 &, T3 &>()))
{
  return std::tuple<T1...>(a0);
}

template <typename... T1, typename T2, typename T3>
std::tuple<T1...> f102(const std::pair<T2, T3> &&a0)
  requires(
      (sizeof...(T1) == 2) &&
      (std::tuple<T1...>::template __constructible<const T2, const T3>()) &&
      (!std::tuple<T1...>::template __dangles<const T2, const T3>()))
{
  return std::tuple<T1...>(std::move(a0));
}

template <typename... T1, typename T2>
std::tuple<T1...> f103(T2 &&a0)
  requires((std::tuple<T1...>::template __constructible_from_tuple_like<
               T2>()) &&
           (!std::tuple<T1...>::template __use_other_ctor<T2>()) &&
           (!std::tuple<T1...>::template __dangles_from_tuple_like<T2>())) &&
          (!(std::is_same<T2, const T1 &...>::value) &&
           !(std::is_same<T2, const std::tuple<T1...> &>::value) &&
           !(std::is_same<T2, std::tuple<T1...>>::value))
{
  return std::tuple<T1...>(std::forward<T2>(a0));
}

template <typename... T1, typename T2, typename... T3>
std::tuple<T1...> f104(std::allocator_arg_t a0, const T2 &a1,
                       std::tuple<T3...> &a2)
  requires(
      (std::tuple<T1...>::template __constructible<T3 &...>()) &&
      (!std::tuple<T1...>::template __use_other_ctor<std::tuple<T3...> &>()) &&
      (!std::tuple<T1...>::template __dangles<T3 &...>()))
{
  return std::tuple<T1...>(std::move(a0), a1, a2);
}

template <typename... T1, typename T2, typename... T3>
std::tuple<T1...> f105(std::allocator_arg_t a0, const T2 &a1,
                       const std::tuple<T3...> &&a2)
  requires((std::tuple<T1...>::template __constructible<const T3...>()) &&
           (!std::tuple<T1...>::template __use_other_ctor<
               const std::tuple<T3...>>()) &&
           (!std::tuple<T1...>::template __dangles<const T3...>()))
{
  return std::tuple<T1...>(std::move(a0), a1, std::move(a2));
}

template <typename... T1, typename T2, typename T3, typename T4>
std::tuple<T1...> f106(std::allocator_arg_t a0, const T2 &a1,
                       std::pair<T3, T4> &a2)
  requires((sizeof...(T1) == 2) &&
           (std::tuple<T1...>::template __constructible<T3 &, T4 &>()) &&
           (!std::tuple<T1...>::template __dangles<T3 &, T4 &>()))
{
  return std::tuple<T1...>(std::move(a0), a1, a2);
}

template <typename... T1, typename T2, typename T3, typename T4>
std::tuple<T1...> f107(std::allocator_arg_t a0, const T2 &a1,
                       const std::pair<T3, T4> &&a2)
  requires(
      (sizeof...(T1) == 2) &&
      (std::tuple<T1...>::template __constructible<const T3, const T4>()) &&
      (!std::tuple<T1...>::template __dangles<const T3, const T4>()))
{
  return std::tuple<T1...>(std::move(a0), a1, std::move(a2));
}

template <typename... T1, typename T2, typename T3>
std::tuple<T1...> f108(std::allocator_arg_t a0, const T2 &a1, T3 &&a2)
  requires(
      (std::tuple<T1...>::template __constructible_from_tuple_like<T3>()) &&
      (!std::tuple<T1...>::template __use_other_ctor<T3>()) &&
      (!std::tuple<T1...>::template __dangles_from_tuple_like<T3>()))
{
  return std::tuple<T1...>(std::move(a0), a1, std::forward<T3>(a2));
}

template <typename... T1, typename... T2>
const std::tuple<T1...> &f109(const std::tuple<T1...> &a0,
                              const std::tuple<T2...> &a1)
  requires((std::tuple<T1...>::template __const_assignable<const T2 &...>()))
{
  return a0.template operator= <T2...>(a1);
}

template <typename... T1, typename... T2>
const std::tuple<T1...> &f110(const std::tuple<T1...> &a0,
                              std::tuple<T2...> &&a1)
  requires((std::tuple<T1...>::template __const_assignable<T2...>()))
{
  return a0.template operator= <T2...>(std::move(a1));
}

template <typename... T1, typename T2, typename T3>
const std::tuple<T1...> &f111(const std::tuple<T1...> &a0,
                              const std::pair<T2, T3> &a1)
  requires(
      (std::tuple<T1...>::template __const_assignable<const T2 &, const T3>()))
{
  return a0.template operator= <T2, T3>(a1);
}

template <typename... T1, typename T2, typename T3>
const std::tuple<T1...> &f112(const std::tuple<T1...> &a0,
                              std::pair<T2, T3> &&a1)
  requires((std::tuple<T1...>::template __const_assignable<T2, T3>()))
{
  return a0.template operator= <T2, T3>(std::move(a1));
}

template <typename... T1, typename T2>
std::tuple<T1...> &f113(std::tuple<T1...> &a0, T2 &&a1)
  requires((std::tuple<T1...>::template __assignable_from_tuple_like<T2>())) &&
          (!(std::is_same<T2, const std::tuple<T1...> &>::value) &&
           !(std::is_same<T2, std::tuple<T1...>>::value))
{
  return a0.template operator= <T2>(std::forward<T2>(a1));
}

template <typename... T1, typename T2>
const std::tuple<T1...> &f114(const std::tuple<T1...> &a0, T2 &&a1)
  requires((std::tuple<T1...>::template __const_assignable_from_tuple_like<
              T2>())) &&
          (!(std::is_same<T2, const std::tuple<T1...> &>::value) &&
           !(std::is_same<T2, std::tuple<T1...>>::value))
{
  return a0.template operator= <T2>(std::forward<T2>(a1));
}

template <typename... T1, typename T2>
bool f115(const std::tuple<T1...> &a0, const T2 &a1)
  requires((!std::__is_tuple_v<T2>))
{
  return operator==(a0, a1);
}

template <typename... T1> auto f116(T1 &&...a0) {
  return std::tuple_cat<T1...>(std::forward<T1>(a0)...);
}
