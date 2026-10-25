#include <initializer_list>
#include <unordered_map>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4, typename T5>
bool f200(const std::unordered_map<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.contains(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6,
          std::__enable_if_t<
              std::is_constructible<std::pair<const T1, T2>, T6>::value, int>
              T7 = 0>
std::pair<typename std::unordered_map<T1, T2, T3, T4, T5>::iterator, bool>
f201(std::unordered_map<T1, T2, T3, T4, T5> &a0, T6 &&a1)
  requires(
      !(std::is_same<T6, const typename std::unordered_map<
                             T1, T2, T3, T4, T5>::value_type &>::value) &&
      !(std::is_same<T6, std::initializer_list<typename std::unordered_map<
                             T1, T2, T3, T4, T5>::value_type>>::value) &&
      !(std::is_same<T6, typename std::unordered_map<T1, T2, T3, T4,
                                                     T5>::value_type>::value) &&
      !(std::is_same<T6, typename std::unordered_map<T1, T2, T3, T4,
                                                     T5>::node_type>::value))
{
  return a0.template insert<T6>(std::forward<T6>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6,
          std::__enable_if_t<
              std::is_constructible<std::pair<const T1, T2>, T6>::value, int>
              T7 = 0>
typename std::unordered_map<T1, T2, T3, T4, T5>::iterator
f202(std::unordered_map<T1, T2, T3, T4, T5> &a0,
     typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator a1,
     T6 &&a2)
  requires(
      !(std::is_same<T6, const typename std::unordered_map<
                             T1, T2, T3, T4, T5>::value_type &>::value) &&
      !(std::is_same<T6, typename std::unordered_map<T1, T2, T3, T4,
                                                     T5>::value_type>::value) &&
      !(std::is_same<T6, typename std::unordered_map<T1, T2, T3, T4,
                                                     T5>::node_type>::value))
{
  return a0.template insert<T6>(
      std::forward<
          typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator>(a1),
      std::forward<T6>(a2));
}

template <
    typename T1, typename T2, typename T3, typename T4, typename T5,
    typename T6, typename T7 = T3, typename T8 = T4,
    std::enable_if_t<std::__is_transparent_v<std::__type_identity_t<T3>> &&
                     std::__is_transparent_v<std::__type_identity_t<T4>>> *T9 =
        nullptr>
typename std::unordered_map<T1, T2, T3, T4, T5>::iterator
f203(std::unordered_map<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(!(std::is_same<T6, typename std::unordered_map<
                                  T1, T2, T3, T4, T5>::key_type>::value))
{
  return a0.template find<T6>(a1);
}

template <
    typename T1, typename T2, typename T3, typename T4, typename T5,
    typename T6, typename T7 = T3, typename T8 = T4,
    std::enable_if_t<std::__is_transparent_v<std::__type_identity_t<T3>> &&
                     std::__is_transparent_v<std::__type_identity_t<T4>>> *T9 =
        nullptr>
typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator
f204(const std::unordered_map<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(!(std::is_same<T6, typename std::unordered_map<
                                  T1, T2, T3, T4, T5>::key_type>::value))
{
  return a0.template find<T6>(a1);
}

template <
    typename T1, typename T2, typename T3, typename T4, typename T5,
    typename T6, typename T7 = T3, typename T8 = T4,
    std::enable_if_t<std::__is_transparent_v<std::__type_identity_t<T3>> &&
                     std::__is_transparent_v<std::__type_identity_t<T4>>> *T9 =
        nullptr>
typename std::unordered_map<T1, T2, T3, T4, T5>::size_type
f205(const std::unordered_map<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(!(std::is_same<T6, typename std::unordered_map<
                                  T1, T2, T3, T4, T5>::key_type>::value))
{
  return a0.template count<T6>(a1);
}

template <
    typename T1, typename T2, typename T3, typename T4, typename T5,
    typename T6, typename T7 = T3, typename T8 = T4,
    std::enable_if_t<std::__is_transparent_v<std::__type_identity_t<T3>> &&
                     std::__is_transparent_v<std::__type_identity_t<T4>>> *T9 =
        nullptr>
bool f206(const std::unordered_map<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(!(std::is_same<T6, typename std::unordered_map<
                                  T1, T2, T3, T4, T5>::key_type>::value))
{
  return a0.template contains<T6>(a1);
}

template <
    typename T1, typename T2, typename T3, typename T4, typename T5,
    typename T6, typename T7 = T3, typename T8 = T4,
    std::enable_if_t<std::__is_transparent_v<std::__type_identity_t<T3>> &&
                     std::__is_transparent_v<std::__type_identity_t<T4>>> *T9 =
        nullptr>
std::pair<typename std::unordered_map<T1, T2, T3, T4, T5>::iterator,
          typename std::unordered_map<T1, T2, T3, T4, T5>::iterator>
f207(std::unordered_map<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(!(std::is_same<T6, typename std::unordered_map<
                                  T1, T2, T3, T4, T5>::key_type>::value))
{
  return a0.template equal_range<T6>(a1);
}

template <
    typename T1, typename T2, typename T3, typename T4, typename T5,
    typename T6, typename T7 = T3, typename T8 = T4,
    std::enable_if_t<std::__is_transparent_v<std::__type_identity_t<T3>> &&
                     std::__is_transparent_v<std::__type_identity_t<T4>>> *T9 =
        nullptr>
std::pair<typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator,
          typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator>
f208(const std::unordered_map<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(!(std::is_same<T6, typename std::unordered_map<
                                  T1, T2, T3, T4, T5>::key_type>::value))
{
  return a0.template equal_range<T6>(a1);
}
