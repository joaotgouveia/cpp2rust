#include <array>
#include <compare>
#include <tuple>
#include <type_traits>
#include <utility>

template <typename... T1>
std::enable_if_t<std::__all<std::is_swappable_v<const T1>...>::value, void>
f172(const std::tuple<T1...> &a0, const std::tuple<T1...> &a1) {
  return std::swap<T1...>(a0, a1);
}

template <typename T1>
std::strong_ordering f173(const std::tuple<> &a0, const T1 &a1)
  requires((std::tuple_size_v<T1> == 0)) && (std::__tuple_like_no_tuple<T1>)
{
  return operator<=>(a0, a1);
}

template <typename T1>
bool f174(const std::tuple<> &a0, const T1 &a1)
  requires(std::__tuple_like_no_tuple<T1>)
{
  return operator==(a0, a1);
}

template <typename... T1>
const std::tuple<T1...> &f175(const std::tuple<T1...> &a0,
                              const std::tuple<T1...> &a1)
  requires((std::_And<std::is_copy_assignable<const T1>...>::value))
{
  return a0.operator=(a1);
}

template <typename... T1>
const std::tuple<T1...> &f176(const std::tuple<T1...> &a0,
                              std::tuple<T1...> &&a1)
  requires((std::_And<std::is_assignable<const T1 &, T1>...>::value))
{
  return a0.operator=(std::move(a1));
}

template <typename... T1>
void f177(const std::tuple<T1...> &a0, const std::tuple<T1...> &a1) {
  return a0.swap(a1);
}

template <
    typename... T1, typename... T2,
    std::enable_if_t<std::tuple<T1...>::template _EnableCtorFromUTypesTuple<
        std::tuple<T2...> &>::value> *T3 = nullptr>
std::tuple<T1...> f178(std::tuple<T2...> &a0) {
  return std::tuple<T1...>(a0);
}

template <
    typename... T1, typename T2, typename... T3,
    std::enable_if_t<std::tuple<T1...>::template _EnableCtorFromUTypesTuple<
        std::tuple<T3...> &>::value> *T4 = nullptr>
std::tuple<T1...> f179(std::allocator_arg_t a0, const T2 &a1,
                       std::tuple<T3...> &a2) {
  return std::tuple<T1...>(std::move(a0), a1, a2);
}

template <
    typename... T1, typename... T2,
    std::enable_if_t<std::tuple<T1...>::template _EnableCtorFromUTypesTuple<
        const std::tuple<T2...> &&>::value> *T3 = nullptr>
std::tuple<T1...> f180(const std::tuple<T2...> &&a0) {
  return std::tuple<T1...>(std::move(a0));
}

template <
    typename... T1, typename T2, typename... T3,
    std::enable_if_t<std::tuple<T1...>::template _EnableCtorFromUTypesTuple<
        const std::tuple<T3...> &&>::value> *T4 = nullptr>
std::tuple<T1...> f181(std::allocator_arg_t a0, const T2 &a1,
                       const std::tuple<T3...> &&a2) {
  return std::tuple<T1...>(std::move(a0), a1, std::move(a2));
}

template <typename... T1, typename T2, typename T3,
          std::enable_if_t<std::tuple<T1...>::template _EnableCtorFromPair<
              std::pair<T2, T3> &>::value> *T4 = nullptr>
std::tuple<T1...> f182(std::pair<T2, T3> &a0) {
  return std::tuple<T1...>(a0);
}

template <typename... T1, typename T2, typename T3, typename T4,
          std::enable_if_t<std::tuple<T1...>::template _EnableCtorFromPair<
              std::pair<T3, T4> &>::value> *T5 = nullptr>
std::tuple<T1...> f183(std::allocator_arg_t a0, const T2 &a1,
                       std::pair<T3, T4> &a2) {
  return std::tuple<T1...>(std::move(a0), a1, a2);
}

template <typename... T1, typename T2, typename T3,
          std::enable_if_t<std::tuple<T1...>::template _EnableCtorFromPair<
              const std::pair<T2, T3> &&>::value> *T4 = nullptr>
std::tuple<T1...> f184(const std::pair<T2, T3> &&a0) {
  return std::tuple<T1...>(std::move(a0));
}

template <typename... T1, typename T2, typename T3, typename T4,
          std::enable_if_t<std::tuple<T1...>::template _EnableCtorFromPair<
              const std::pair<T3, T4> &&>::value> *T5 = nullptr>
std::tuple<T1...> f185(std::allocator_arg_t a0, const T2 &a1,
                       const std::pair<T3, T4> &&a2) {
  return std::tuple<T1...>(std::move(a0), a1, std::move(a2));
}

template <typename... T1, typename... T2,
          std::enable_if_t<
              std::_And<std::_BoolConstant<sizeof...(T1) == sizeof...(T2)>,
                        std::is_assignable<const T1 &, const T2 &>...>::value>
              *T3 = nullptr>
const std::tuple<T1...> &f186(const std::tuple<T1...> &a0,
                              const std::tuple<T2...> &a1) {
  return a0.template operator= <T2...>(a1);
}

template <typename... T1, typename... T2,
          std::enable_if_t<std::_And<
              std::_BoolConstant<sizeof...(T1) == sizeof...(T2)>,
              std::is_assignable<const T1 &, T2>...>::value> *T3 = nullptr>
const std::tuple<T1...> &f187(const std::tuple<T1...> &a0,
                              std::tuple<T2...> &&a1) {
  return a0.template operator= <T2...>(std::move(a1));
}

template <typename... T1, typename T2, typename T3,
          std::enable_if_t<std::tuple<T1...>::template _EnableAssignFromPair<
              true, const std::pair<T2, T3> &>::value> *T4 = nullptr>
const std::tuple<T1...> &f188(const std::tuple<T1...> &a0,
                              const std::pair<T2, T3> &a1) {
  return a0.template operator= <T2, T3>(a1);
}

template <typename... T1, typename T2, typename T3,
          std::enable_if_t<std::tuple<T1...>::template _EnableAssignFromPair<
              true, std::pair<T2, T3> &&>::value> *T4 = nullptr>
const std::tuple<T1...> &f189(const std::tuple<T1...> &a0,
                              std::pair<T2, T3> &&a1) {
  return a0.template operator= <T2, T3>(std::move(a1));
}

template <typename... T1, typename T2>
bool f190(const std::tuple<T1...> &a0, const T2 &a1)
  requires(std::__tuple_like_no_tuple<T2>)
{
  return operator==(a0, a1);
}
