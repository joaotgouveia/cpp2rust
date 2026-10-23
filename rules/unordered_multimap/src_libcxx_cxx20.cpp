#include <initializer_list>
#include <unordered_map>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4, typename T5>
bool f121(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.contains(a1);
}

template <
    typename T1, typename T2, typename T3, typename T4, typename T5,
    typename T6, typename T7 = T3, typename T8 = T4,
    std::enable_if_t<std::__is_transparent_v<std::__type_identity_t<T3>> &&
                     std::__is_transparent_v<std::__type_identity_t<T4>>> *T9 =
        nullptr>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::iterator
f122(std::unordered_multimap<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return a0.template find<T6>(a1);
}

template <
    typename T1, typename T2, typename T3, typename T4, typename T5,
    typename T6, typename T7 = T3, typename T8 = T4,
    std::enable_if_t<std::__is_transparent_v<std::__type_identity_t<T3>> &&
                     std::__is_transparent_v<std::__type_identity_t<T4>>> *T9 =
        nullptr>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::const_iterator
f123(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return a0.template find<T6>(a1);
}

template <
    typename T1, typename T2, typename T3, typename T4, typename T5,
    typename T6, typename T7 = T3, typename T8 = T4,
    std::enable_if_t<std::__is_transparent_v<std::__type_identity_t<T3>> &&
                     std::__is_transparent_v<std::__type_identity_t<T4>>> *T9 =
        nullptr>
typename std::unordered_multimap<T1, T2, T3, T4, T5>::size_type
f124(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return a0.template count<T6>(a1);
}

template <
    typename T1, typename T2, typename T3, typename T4, typename T5,
    typename T6, typename T7 = T3, typename T8 = T4,
    std::enable_if_t<std::__is_transparent_v<std::__type_identity_t<T3>> &&
                     std::__is_transparent_v<std::__type_identity_t<T4>>> *T9 =
        nullptr>
bool f125(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return a0.template contains<T6>(a1);
}

template <
    typename T1, typename T2, typename T3, typename T4, typename T5,
    typename T6, typename T7 = T3, typename T8 = T4,
    std::enable_if_t<std::__is_transparent_v<std::__type_identity_t<T3>> &&
                     std::__is_transparent_v<std::__type_identity_t<T4>>> *T9 =
        nullptr>
std::pair<typename std::unordered_multimap<T1, T2, T3, T4, T5>::iterator,
          typename std::unordered_multimap<T1, T2, T3, T4, T5>::iterator>
f126(std::unordered_multimap<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return a0.template equal_range<T6>(a1);
}

template <
    typename T1, typename T2, typename T3, typename T4, typename T5,
    typename T6, typename T7 = T3, typename T8 = T4,
    std::enable_if_t<std::__is_transparent_v<std::__type_identity_t<T3>> &&
                     std::__is_transparent_v<std::__type_identity_t<T4>>> *T9 =
        nullptr>
std::pair<typename std::unordered_multimap<T1, T2, T3, T4, T5>::const_iterator,
          typename std::unordered_multimap<T1, T2, T3, T4, T5>::const_iterator>
f127(const std::unordered_multimap<T1, T2, T3, T4, T5> &a0, const T6 &a1) {
  return a0.template equal_range<T6>(a1);
}
