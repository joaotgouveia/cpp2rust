#include <algorithm>
#include <tuple>
#include <utility>

template <typename T1, typename T2>
decltype(auto) f118(T1 &&a0, T2 &&a1)
  requires(std::__tuple_like<T2>)
{
  return std::apply<T1, T2>(std::forward<T1>(a0), std::forward<T2>(a1));
}

template <typename T1, typename T2>
T1 f119(T2 &&a0)
  requires(std::__tuple_like<T2>)
{
  return std::make_from_tuple<T1, T2>(std::forward<T2>(a0));
}

template <typename... T1>
void f120(const std::tuple<T1...> &a0, const std::tuple<T1...> &a1)
  requires((std::is_swappable_v<const T1> && ...))
{
  return std::swap<T1...>(a0, a1);
}

template <typename... T1>
const std::tuple<T1...> &f121(const std::tuple<T1...> &a0,
                              const std::tuple<T1...> &a1)
  requires((std::tuple<T1...>::template __const_assignable<const T1 &...>()))
{
  return a0.operator=(a1);
}

template <typename... T1>
const std::tuple<T1...> &f122(const std::tuple<T1...> &a0,
                              std::tuple<T1...> &&a1)
  requires((std::tuple<T1...>::template __const_assignable<T1...>()))
{
  return a0.operator=(std::move(a1));
}

template <typename... T1>
void f123(const std::tuple<T1...> &a0, const std::tuple<T1...> &a1)
  requires((std::is_swappable_v<const T1> && ...))
{
  return a0.swap(a1);
}

template <typename... T1, typename... T2>
std::tuple<T1...> f124(std::tuple<T2...> &a0)
  requires(
      (std::tuple<T1...>::template __constructible<T2 &...>()) &&
      (!std::tuple<T1...>::template __use_other_ctor<std::tuple<T2...> &>()) &&
      (!std::tuple<T1...>::template __dangles<T2 &...>()))
{
  return std::tuple<T1...>(a0);
}

template <typename... T1, typename... T2>
std::tuple<T1...> f125(const std::tuple<T2...> &&a0)
  requires((std::tuple<T1...>::template __constructible<const T2...>()) &&
           (!std::tuple<T1...>::template __use_other_ctor<
               const std::tuple<T2...>>()) &&
           (!std::tuple<T1...>::template __dangles<const T2...>()))
{
  return std::tuple<T1...>(std::move(a0));
}

template <typename... T1, typename T2, typename T3>
std::tuple<T1...> f126(std::pair<T2, T3> &a0)
  requires((sizeof...(T1) == 2) &&
           (std::tuple<T1...>::template __constructible<T2 &, T3 &>()) &&
           (!std::tuple<T1...>::template __dangles<T2 &, T3 &>()))
{
  return std::tuple<T1...>(a0);
}

template <typename... T1, typename T2, typename T3>
std::tuple<T1...> f127(const std::pair<T2, T3> &&a0)
  requires(
      (sizeof...(T1) == 2) &&
      (std::tuple<T1...>::template __constructible<const T2, const T3>()) &&
      (!std::tuple<T1...>::template __dangles<const T2, const T3>()))
{
  return std::tuple<T1...>(std::move(a0));
}

template <typename... T1, typename T2>
std::tuple<T1...> f128(T2 &&a0)
  requires((std::tuple<T1...>::template __constructible_from_tuple_like<
               T2>()) &&
           (!std::tuple<T1...>::template __use_other_ctor<T2>()) &&
           (!std::tuple<T1...>::template __dangles_from_tuple_like<T2>())) &&
          (std::__eligible_tuple_like<T2, std::tuple<T1...>>) &&
          (!(std::is_same<T2, const std::tuple<T1...> &>::value) &&
           !(std::is_same<T2, std::tuple<T1...>>::value))
{
  return std::tuple<T1...>(std::forward<T2>(a0));
}

template <typename... T1, typename T2, typename... T3>
std::tuple<T1...> f129(std::allocator_arg_t a0, const T2 &a1,
                       std::tuple<T3...> &a2)
  requires(
      (std::tuple<T1...>::template __constructible<T3 &...>()) &&
      (!std::tuple<T1...>::template __use_other_ctor<std::tuple<T3...> &>()) &&
      (!std::tuple<T1...>::template __dangles<T3 &...>()))
{
  return std::tuple<T1...>(std::move(a0), a1, a2);
}

template <typename... T1, typename T2, typename... T3>
std::tuple<T1...> f130(std::allocator_arg_t a0, const T2 &a1,
                       const std::tuple<T3...> &&a2)
  requires((std::tuple<T1...>::template __constructible<const T3...>()) &&
           (!std::tuple<T1...>::template __use_other_ctor<
               const std::tuple<T3...>>()) &&
           (!std::tuple<T1...>::template __dangles<const T3...>()))
{
  return std::tuple<T1...>(std::move(a0), a1, std::move(a2));
}

template <typename... T1, typename T2, typename T3, typename T4>
std::tuple<T1...> f131(std::allocator_arg_t a0, const T2 &a1,
                       std::pair<T3, T4> &a2)
  requires((sizeof...(T1) == 2) &&
           (std::tuple<T1...>::template __constructible<T3 &, T4 &>()) &&
           (!std::tuple<T1...>::template __dangles<T3 &, T4 &>()))
{
  return std::tuple<T1...>(std::move(a0), a1, a2);
}

template <typename... T1, typename T2, typename T3, typename T4>
std::tuple<T1...> f132(std::allocator_arg_t a0, const T2 &a1,
                       const std::pair<T3, T4> &&a2)
  requires(
      (sizeof...(T1) == 2) &&
      (std::tuple<T1...>::template __constructible<const T3, const T4>()) &&
      (!std::tuple<T1...>::template __dangles<const T3, const T4>()))
{
  return std::tuple<T1...>(std::move(a0), a1, std::move(a2));
}

template <typename... T1, typename T2, typename T3>
std::tuple<T1...> f133(std::allocator_arg_t a0, const T2 &a1, T3 &&a2)
  requires((std::tuple<T1...>::template __constructible_from_tuple_like<
               T3>()) &&
           (!std::tuple<T1...>::template __use_other_ctor<T3>()) &&
           (!std::tuple<T1...>::template __dangles_from_tuple_like<T3>())) &&
          (std::__eligible_tuple_like<T3, std::tuple<T1...>>)
{
  return std::tuple<T1...>(std::move(a0), a1, std::forward<T3>(a2));
}

template <typename... T1, typename... T2>
const std::tuple<T1...> &f134(const std::tuple<T1...> &a0,
                              const std::tuple<T2...> &a1)
  requires((std::tuple<T1...>::template __const_assignable<const T2 &...>()))
{
  return a0.template operator= <T2...>(a1);
}

template <typename... T1, typename... T2>
const std::tuple<T1...> &f135(const std::tuple<T1...> &a0,
                              std::tuple<T2...> &&a1)
  requires((std::tuple<T1...>::template __const_assignable<T2...>()))
{
  return a0.template operator= <T2...>(std::move(a1));
}

template <typename... T1, typename T2, typename T3>
const std::tuple<T1...> &f136(const std::tuple<T1...> &a0,
                              const std::pair<T2, T3> &a1)
  requires(
      (std::tuple<T1...>::template __const_assignable<const T2 &, const T3>()))
{
  return a0.template operator= <T2, T3>(a1);
}

template <typename... T1, typename T2, typename T3>
const std::tuple<T1...> &f137(const std::tuple<T1...> &a0,
                              std::pair<T2, T3> &&a1)
  requires((std::tuple<T1...>::template __const_assignable<T2, T3>()))
{
  return a0.template operator= <T2, T3>(std::move(a1));
}

template <typename... T1, typename T2>
std::tuple<T1...> &f138(std::tuple<T1...> &a0, T2 &&a1)
  requires((std::tuple<T1...>::template __assignable_from_tuple_like<T2>())) &&
          (std::__eligible_tuple_like<T2, std::tuple<T1...>>) &&
          (!(std::is_same<T2, const std::tuple<T1...> &>::value) &&
           !(std::is_same<T2, std::tuple<T1...>>::value))
{
  return a0.template operator= <T2>(std::forward<T2>(a1));
}

template <typename... T1, typename T2>
const std::tuple<T1...> &f139(const std::tuple<T1...> &a0, T2 &&a1)
  requires((std::tuple<T1...>::template __const_assignable_from_tuple_like<
              T2>())) &&
          (std::__eligible_tuple_like<T2, std::tuple<T1...>>) &&
          (!(std::is_same<T2, const std::tuple<T1...> &>::value) &&
           !(std::is_same<T2, std::tuple<T1...>>::value))
{
  return a0.template operator= <T2>(std::forward<T2>(a1));
}

template <typename... T1, typename T2>
bool f140(const std::tuple<T1...> &a0, const T2 &a1)
  requires((!std::__is_tuple_v<T2>)) && (std::__tuple_like<T2>)
{
  return operator==(a0, a1);
}

template <typename... T1>
auto f141(T1 &&...a0)
  requires((std::__tuple_like<T1> && ...))
{
  return std::tuple_cat<T1...>(std::forward<T1>(a0)...);
}
