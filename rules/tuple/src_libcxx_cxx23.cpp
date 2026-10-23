#include <array>
#include <tuple>
#include <utility>

template <typename... T1>
const std::tuple<T1...> &f129(const std::tuple<T1...> &a0,
                              const std::tuple<T1...> &a1)
  requires((std::_And<std::is_copy_assignable<const T1>...>::value))
{
  return a0.operator=(a1);
}

template <typename... T1>
const std::tuple<T1...> &f130(const std::tuple<T1...> &a0,
                              std::tuple<T1...> &&a1)
  requires((std::_And<std::is_assignable<const T1 &, T1>...>::value))
{
  return a0.operator=(std::move(a1));
}

template <typename... T1>
void f131(const std::tuple<T1...> &a0, const std::tuple<T1...> &a1) {
  return a0.swap(a1);
}

template <
    typename... T1, typename... T2,
    std::enable_if_t<std::tuple<T1...>::template _EnableCtorFromUTypesTuple<
        std::tuple<T2...> &>::value> *T3 = nullptr>
std::tuple<T1...> f132(std::tuple<T2...> &a0) {
  return std::tuple<T1...>(a0);
}

template <
    typename... T1, typename T2, typename... T3,
    std::enable_if_t<std::tuple<T1...>::template _EnableCtorFromUTypesTuple<
        std::tuple<T3...> &>::value> *T4 = nullptr>
std::tuple<T1...> f133(std::allocator_arg_t a0, const T2 &a1,
                       std::tuple<T3...> &a2) {
  return std::tuple<T1...>(std::move(a0), a1, a2);
}

template <
    typename... T1, typename... T2,
    std::enable_if_t<std::tuple<T1...>::template _EnableCtorFromUTypesTuple<
        const std::tuple<T2...> &&>::value> *T3 = nullptr>
std::tuple<T1...> f134(const std::tuple<T2...> &&a0) {
  return std::tuple<T1...>(std::move(a0));
}

template <
    typename... T1, typename T2, typename... T3,
    std::enable_if_t<std::tuple<T1...>::template _EnableCtorFromUTypesTuple<
        const std::tuple<T3...> &&>::value> *T4 = nullptr>
std::tuple<T1...> f135(std::allocator_arg_t a0, const T2 &a1,
                       const std::tuple<T3...> &&a2) {
  return std::tuple<T1...>(std::move(a0), a1, std::move(a2));
}

template <typename... T1, typename T2, typename T3,
          std::enable_if_t<std::tuple<T1...>::template _EnableCtorFromPair<
              std::pair<T2, T3> &>::value> *T4 = nullptr>
std::tuple<T1...> f136(std::pair<T2, T3> &a0) {
  return std::tuple<T1...>(a0);
}

template <typename... T1, typename T2, typename T3, typename T4,
          std::enable_if_t<std::tuple<T1...>::template _EnableCtorFromPair<
              std::pair<T3, T4> &>::value> *T5 = nullptr>
std::tuple<T1...> f137(std::allocator_arg_t a0, const T2 &a1,
                       std::pair<T3, T4> &a2) {
  return std::tuple<T1...>(std::move(a0), a1, a2);
}

template <typename... T1, typename T2, typename T3,
          std::enable_if_t<std::tuple<T1...>::template _EnableCtorFromPair<
              const std::pair<T2, T3> &&>::value> *T4 = nullptr>
std::tuple<T1...> f138(const std::pair<T2, T3> &&a0) {
  return std::tuple<T1...>(std::move(a0));
}

template <typename... T1, typename T2, typename T3, typename T4,
          std::enable_if_t<std::tuple<T1...>::template _EnableCtorFromPair<
              const std::pair<T3, T4> &&>::value> *T5 = nullptr>
std::tuple<T1...> f139(std::allocator_arg_t a0, const T2 &a1,
                       const std::pair<T3, T4> &&a2) {
  return std::tuple<T1...>(std::move(a0), a1, std::move(a2));
}

template <typename... T1, typename... T2,
          std::enable_if_t<
              std::_And<std::_BoolConstant<sizeof...(T1) == sizeof...(T2)>,
                        std::is_assignable<const T1 &, const T2 &>...>::value>
              *T3 = nullptr>
const std::tuple<T1...> &f140(const std::tuple<T1...> &a0,
                              const std::tuple<T2...> &a1) {
  return a0.template operator= <T2...>(a1);
}

template <typename... T1, typename... T2,
          std::enable_if_t<std::_And<
              std::_BoolConstant<sizeof...(T1) == sizeof...(T2)>,
              std::is_assignable<const T1 &, T2>...>::value> *T3 = nullptr>
const std::tuple<T1...> &f141(const std::tuple<T1...> &a0,
                              std::tuple<T2...> &&a1) {
  return a0.template operator= <T2...>(std::move(a1));
}

template <typename... T1, typename T2, typename T3,
          std::enable_if_t<std::tuple<T1...>::template _EnableAssignFromPair<
              true, const std::pair<T2, T3> &>::value> *T4 = nullptr>
const std::tuple<T1...> &f142(const std::tuple<T1...> &a0,
                              const std::pair<T2, T3> &a1) {
  return a0.template operator= <T2, T3>(a1);
}

template <typename... T1, typename T2, typename T3,
          std::enable_if_t<std::tuple<T1...>::template _EnableAssignFromPair<
              true, std::pair<T2, T3> &&>::value> *T4 = nullptr>
const std::tuple<T1...> &f143(const std::tuple<T1...> &a0,
                              std::pair<T2, T3> &&a1) {
  return a0.template operator= <T2, T3>(std::move(a1));
}
