// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include <algorithm>
#include <tuple>
#include <utility>

template <std::size_t T1, typename... T2>
typename std::tuple_element<T1, std::tuple<T2...>>::type &
f6(std::tuple<T2...> &a0) {
  return std::get<T1>(a0);
}

template <typename... T1> auto f12(T1 &&...a0) {
  return std::make_tuple(std::forward<T1>(a0)...);
}

template <typename... T1, typename... T2>
bool f14(const std::tuple<T1...> &a0, const std::tuple<T2...> &a1) {
  return std::operator==(a0, a1);
}

template <typename... T1> using t2 = std::tuple<T1...>;

template <typename... T1>
std::tuple<T1...> f27()
  requires((std::is_default_constructible_v<T1> && ...))
{
  return std::tuple<T1...>();
}

template <typename... T1, typename T2 = void>
std::tuple<T1...> f36(const T1 &...a0) {
  return std::tuple<T1...>(a0...);
}

template <typename... T1, typename... T2>
std::tuple<T1...> f39(std::tuple<T2...> &&a0) {
  return std::tuple<T1...>(std::move(a0));
}

template <typename... T1, typename T2>
std::tuple<T1...> f46(T2 &&a0)
  requires(!(std::is_same<T2, const std::tuple<T1...> &>::value) &&
           !(std::is_same<T2, std::tuple<T1...>>::value))
{
  return std::tuple<T1...>(std::forward<T2>(a0));
}
