#include <algorithm>
#include <flat_map>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_multimap<T1, T2, T3, T4, T5>::size_type
f2(std::flat_multimap<T1, T2, T3, T4, T5> &a0, T6 a1) {
  return std::erase_if<T1, T2, T3, T4, T5, T6>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t14 = typename std::flat_map<T1, T2, T3, T4, T5>::iterator;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t15 = typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t16 = typename std::flat_map<T1, T2, T3, T4, T5>::reverse_iterator;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
using t17 = typename std::flat_map<T1, T2, T3, T4, T5>::const_reverse_iterator;

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type &
f3(std::flat_map<T1, T2, T3, T4, T5> &a0,
   const typename std::flat_map<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type &
f4(std::flat_map<T1, T2, T3, T4, T5> &a0,
   typename std::flat_map<T1, T2, T3, T4, T5>::key_type &&a1) {
  return a0.operator[](std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type &
f5(std::flat_map<T1, T2, T3, T4, T5> &a0,
   const typename std::flat_map<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.at(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
const typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type &
f6(const std::flat_map<T1, T2, T3, T4, T5> &a0,
   const typename std::flat_map<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.at(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type &
f7(std::flat_map<T1, T2, T3, T4, T5> &a0, T6 &&a1)
  requires(std::__transparent_comparator<T3>)
{
  return a0.template operator[]<T6>(std::forward<T6>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type &
f8(std::flat_map<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(std::same_as<T6, T1> || std::__transparent_comparator<T3>)
{
  return a0.template at<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
const typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type &
f9(const std::flat_map<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(std::same_as<T6, T1> || std::__transparent_comparator<T3>)
{
  return a0.template at<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename... T6>
std::pair<typename std::flat_map<T1, T2, T3, T4, T5>::iterator, bool>
f10(std::flat_map<T1, T2, T3, T4, T5> &a0,
    const typename std::flat_map<T1, T2, T3, T4, T5>::key_type &a1, T6 &&...a2)
  requires(std::is_constructible_v<
           typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type, T6...>)
{
  return a0.template try_emplace<T6...>(a1, std::forward<T6>(a2)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename... T6>
std::pair<typename std::flat_map<T1, T2, T3, T4, T5>::iterator, bool>
f11(std::flat_map<T1, T2, T3, T4, T5> &a0,
    typename std::flat_map<T1, T2, T3, T4, T5>::key_type &&a1, T6 &&...a2)
  requires(std::is_constructible_v<
           typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type, T6...>)
{
  return a0.template try_emplace<T6...>(std::move(a1), std::forward<T6>(a2)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename... T7>
std::pair<typename std::flat_map<T1, T2, T3, T4, T5>::iterator, bool>
f12(std::flat_map<T1, T2, T3, T4, T5> &a0, T6 &&a1, T7 &&...a2)
  requires(
      std::__transparent_comparator<T3> &&
      std::is_constructible_v<
          typename std::flat_map<T1, T2, T3, T4, T5>::key_type, T6> &&
      std::is_constructible_v<
          typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type, T7...> &&
      (!std::is_convertible_v<
          T6 &&, typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator>) &&
      (!std::is_convertible_v<
          T6 &&, typename std::flat_map<T1, T2, T3, T4, T5>::iterator>))
{
  return a0.template try_emplace<T6, T7...>(std::forward<T6>(a1),
                                            std::forward<T7>(a2)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename... T6>
typename std::flat_map<T1, T2, T3, T4, T5>::iterator
f13(std::flat_map<T1, T2, T3, T4, T5> &a0,
    typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator a1,
    const typename std::flat_map<T1, T2, T3, T4, T5>::key_type &a2, T6 &&...a3)
  requires(std::is_constructible_v<
           typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type, T6...>)
{
  return a0.template try_emplace<T6...>(a1, a2, std::forward<T6>(a3)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename... T6>
typename std::flat_map<T1, T2, T3, T4, T5>::iterator
f14(std::flat_map<T1, T2, T3, T4, T5> &a0,
    typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator a1,
    typename std::flat_map<T1, T2, T3, T4, T5>::key_type &&a2, T6 &&...a3)
  requires(std::is_constructible_v<
           typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type, T6...>)
{
  return a0.template try_emplace<T6...>(a1, std::move(a2),
                                        std::forward<T6>(a3)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename... T7>
typename std::flat_map<T1, T2, T3, T4, T5>::iterator
f15(std::flat_map<T1, T2, T3, T4, T5> &a0,
    typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator a1, T6 &&a2,
    T7 &&...a3)
  requires(std::__transparent_comparator<T3> &&
           std::is_constructible_v<
               typename std::flat_map<T1, T2, T3, T4, T5>::key_type, T6> &&
           std::is_constructible_v<
               typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type, T7...>)
{
  return a0.template try_emplace<T6, T7...>(a1, std::forward<T6>(a2),
                                            std::forward<T7>(a3)...);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::pair<typename std::flat_map<T1, T2, T3, T4, T5>::iterator, bool>
f16(std::flat_map<T1, T2, T3, T4, T5> &a0,
    const typename std::flat_map<T1, T2, T3, T4, T5>::key_type &a1, T6 &&a2)
  requires(std::is_assignable_v<
               typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type &, T6> &&
           std::is_constructible_v<
               typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type, T6>)
{
  return a0.template insert_or_assign<T6>(a1, std::forward<T6>(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::pair<typename std::flat_map<T1, T2, T3, T4, T5>::iterator, bool>
f17(std::flat_map<T1, T2, T3, T4, T5> &a0,
    typename std::flat_map<T1, T2, T3, T4, T5>::key_type &&a1, T6 &&a2)
  requires(std::is_assignable_v<
               typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type &, T6> &&
           std::is_constructible_v<
               typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type, T6>)
{
  return a0.template insert_or_assign<T6>(std::move(a1), std::forward<T6>(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
std::pair<typename std::flat_map<T1, T2, T3, T4, T5>::iterator, bool>
f18(std::flat_map<T1, T2, T3, T4, T5> &a0, T6 &&a1, T7 &&a2)
  requires((std::same_as<std::remove_cvref_t<T6>, T1> ||
            std::__transparent_comparator<T3>) &&
           std::is_constructible_v<
               typename std::flat_map<T1, T2, T3, T4, T5>::key_type, T6> &&
           std::is_assignable_v<
               typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type &, T7> &&
           std::is_constructible_v<
               typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type, T7>)
{
  return a0.template insert_or_assign<T6, T7>(std::forward<T6>(a1),
                                              std::forward<T7>(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_map<T1, T2, T3, T4, T5>::iterator
f19(std::flat_map<T1, T2, T3, T4, T5> &a0,
    typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator a1,
    const typename std::flat_map<T1, T2, T3, T4, T5>::key_type &a2, T6 &&a3)
  requires(std::is_assignable_v<
               typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type &, T6> &&
           std::is_constructible_v<
               typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type, T6>)
{
  return a0.template insert_or_assign<T6>(a1, a2, std::forward<T6>(a3));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
typename std::flat_map<T1, T2, T3, T4, T5>::iterator
f20(std::flat_map<T1, T2, T3, T4, T5> &a0,
    typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator a1,
    typename std::flat_map<T1, T2, T3, T4, T5>::key_type &&a2, T6 &&a3)
  requires(std::is_assignable_v<
               typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type &, T6> &&
           std::is_constructible_v<
               typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type, T6>)
{
  return a0.template insert_or_assign<T6>(a1, std::move(a2),
                                          std::forward<T6>(a3));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6, typename T7>
typename std::flat_map<T1, T2, T3, T4, T5>::iterator
f21(std::flat_map<T1, T2, T3, T4, T5> &a0,
    typename std::flat_map<T1, T2, T3, T4, T5>::const_iterator a1, T6 &&a2,
    T7 &&a3)
  requires((std::same_as<std::remove_cvref_t<T6>, T1> ||
            std::__transparent_comparator<T3>) &&
           std::is_constructible_v<
               typename std::flat_map<T1, T2, T3, T4, T5>::key_type, T6> &&
           std::is_assignable_v<
               typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type &, T7> &&
           std::is_constructible_v<
               typename std::flat_map<T1, T2, T3, T4, T5>::mapped_type, T7>)
{
  return a0.template insert_or_assign<T6, T7>(a1, std::forward<T6>(a2),
                                              std::forward<T7>(a3));
}
