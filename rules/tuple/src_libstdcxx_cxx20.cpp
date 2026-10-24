#include <algorithm>
#include <tuple>
#include <utility>

template <typename... T1, typename... T2>
auto f96(const std::tuple<T1...> &a0, const std::tuple<T2...> &a1) {
  return std::operator<=> <T1...>(a0, a1);
}

template <typename... T1>
std::tuple<T1...> f97()
  requires((std::is_default_constructible_v<T1> && ...))
{
  return std::tuple<T1...>();
}

template <typename... T1>
std::tuple<T1...> &f98(std::tuple<T1...> &a0, const std::tuple<T1...> &a1)
  requires((std::tuple<T1...>::template __assignable<const T1 &...>()))
{
  return a0.operator=(a1);
}

template <typename... T1>
std::tuple<T1...> &f99(std::tuple<T1...> &a0, std::tuple<T1...> &&a1)
  requires((std::tuple<T1...>::template __assignable<T1...>()))
{
  return a0.operator=(std::move(a1));
}

template <typename... T1, typename T2 = void>
std::tuple<T1...> f100(const T1 &...a0)
  requires((std::tuple<T1...>::template __constructible<const T1 &...>()))
{
  return std::tuple<T1...>(a0...);
}

template <typename... T1, typename... T2>
std::tuple<T1...> f101(T2 &&...a0)
  requires((std::tuple<T1...>::template __disambiguating_constraint<T2...>()) &&
           (std::tuple<T1...>::template __constructible<T2...>()) &&
           (!std::tuple<T1...>::template __dangles<T2...>()))
{
  return std::tuple<T1...>(std::forward<T2>(a0)...);
}

template <typename... T1, typename... T2>
std::tuple<T1...> f102(const std::tuple<T2...> &a0)
  requires((std::tuple<T1...>::template __constructible<const T2 &...>()) &&
           (!std::tuple<T1...>::template __use_other_ctor<
               const std::tuple<T2...> &>()) &&
           (!std::tuple<T1...>::template __dangles<const T2 &...>()))
{
  return std::tuple<T1...>(a0);
}

template <typename... T1, typename... T2>
std::tuple<T1...> f103(std::tuple<T2...> &&a0)
  requires(
      (std::tuple<T1...>::template __constructible<T2...>()) &&
      (!std::tuple<T1...>::template __use_other_ctor<std::tuple<T2...>>()) &&
      (!std::tuple<T1...>::template __dangles<T2...>()))
{
  return std::tuple<T1...>(std::move(a0));
}

template <typename... T1, typename T2, typename T3>
std::tuple<T1...> f104(const std::pair<T2, T3> &a0)
  requires(
      (sizeof...(T1) == 2) &&
      (std::tuple<T1...>::template __constructible<const T2 &, const T3 &>()) &&
      (!std::tuple<T1...>::template __dangles<const T2 &, const T3 &>()))
{
  return std::tuple<T1...>(a0);
}

template <typename... T1, typename T2, typename T3>
std::tuple<T1...> f105(std::pair<T2, T3> &&a0)
  requires((sizeof...(T1) == 2) &&
           (std::tuple<T1...>::template __constructible<T2, T3>()) &&
           (!std::tuple<T1...>::template __dangles<T2, T3>()))
{
  return std::tuple<T1...>(std::move(a0));
}

template <typename... T1, typename T2>
std::tuple<T1...> f106(std::allocator_arg_t a0, const T2 &a1)
  requires((std::is_default_constructible_v<T1> && ...))
{
  return std::tuple<T1...>(std::move(a0), a1);
}

template <typename... T1, typename T2>
std::tuple<T1...> f107(std::allocator_arg_t a0, const T2 &a1, const T1 &...a2)
  requires((std::tuple<T1...>::template __constructible<const T1 &...>()))
{
  return std::tuple<T1...>(std::move(a0), a1, a2...);
}

template <typename... T1, typename T2, typename... T3>
std::tuple<T1...> f108(std::allocator_arg_t a0, const T2 &a1, T3 &&...a2)
  requires((std::tuple<T1...>::template __disambiguating_constraint<T3...>()) &&
           (std::tuple<T1...>::template __constructible<T3...>()) &&
           (!std::tuple<T1...>::template __dangles<T3...>()))
{
  return std::tuple<T1...>(std::move(a0), a1, std::forward<T3>(a2)...);
}

template <typename... T1, typename T2>
std::tuple<T1...> f109(std::allocator_arg_t a0, const T2 &a1,
                       std::tuple<T1...> &&a2)
  requires((std::tuple<T1...>::template __constructible<T1...>()))
{
  return std::tuple<T1...>(std::move(a0), a1, std::move(a2));
}

template <typename... T1, typename T2, typename... T3>
std::tuple<T1...> f110(std::allocator_arg_t a0, const T2 &a1,
                       const std::tuple<T3...> &a2)
  requires((std::tuple<T1...>::template __constructible<const T3 &...>()) &&
           (!std::tuple<T1...>::template __use_other_ctor<
               const std::tuple<T3...> &>()) &&
           (!std::tuple<T1...>::template __dangles<const T3 &...>()))
{
  return std::tuple<T1...>(std::move(a0), a1, a2);
}

template <typename... T1, typename T2, typename... T3>
std::tuple<T1...> f111(std::allocator_arg_t a0, const T2 &a1,
                       std::tuple<T3...> &&a2)
  requires(
      (std::tuple<T1...>::template __constructible<T3...>()) &&
      (!std::tuple<T1...>::template __use_other_ctor<std::tuple<T3...>>()) &&
      (!std::tuple<T1...>::template __dangles<T3...>()))
{
  return std::tuple<T1...>(std::move(a0), a1, std::move(a2));
}

template <typename... T1, typename T2, typename T3, typename T4>
std::tuple<T1...> f112(std::allocator_arg_t a0, const T2 &a1,
                       const std::pair<T3, T4> &a2)
  requires(
      (sizeof...(T1) == 2) &&
      (std::tuple<T1...>::template __constructible<const T3 &, const T4 &>()) &&
      (!std::tuple<T1...>::template __dangles<const T3 &, const T4 &>()))
{
  return std::tuple<T1...>(std::move(a0), a1, a2);
}

template <typename... T1, typename T2, typename T3, typename T4>
std::tuple<T1...> f113(std::allocator_arg_t a0, const T2 &a1,
                       std::pair<T3, T4> &&a2)
  requires((sizeof...(T1) == 2) &&
           (std::tuple<T1...>::template __constructible<T3, T4>()) &&
           (!std::tuple<T1...>::template __dangles<T3, T4>()))
{
  return std::tuple<T1...>(std::move(a0), a1, std::move(a2));
}

template <typename... T1, typename... T2>
std::tuple<T1...> &f114(std::tuple<T1...> &a0, const std::tuple<T2...> &a1)
  requires((std::tuple<T1...>::template __assignable<const T2 &...>()))
{
  return a0.template operator= <T2...>(a1);
}

template <typename... T1, typename... T2>
std::tuple<T1...> &f115(std::tuple<T1...> &a0, std::tuple<T2...> &&a1)
  requires((std::tuple<T1...>::template __assignable<T2...>()))
{
  return a0.template operator= <T2...>(std::move(a1));
}

template <typename... T1, typename T2, typename T3>
std::tuple<T1...> &f116(std::tuple<T1...> &a0, const std::pair<T2, T3> &a1)
  requires((std::tuple<T1...>::template __assignable<const T2 &, const T3 &>()))
{
  return a0.template operator= <T2, T3>(a1);
}

template <typename... T1, typename T2, typename T3>
std::tuple<T1...> &f117(std::tuple<T1...> &a0, std::pair<T2, T3> &&a1)
  requires((std::tuple<T1...>::template __assignable<T2, T3>()))
{
  return a0.template operator= <T2, T3>(std::move(a1));
}
