#include <initializer_list>
#include <unordered_set>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
bool f119(const std::unordered_multiset<T1, T2, T3, T4> &a0, const T1 &a1) {
  return a0.contains(a1);
}

template <
    typename T1, typename T2, typename T3, typename T4, typename T5,
    typename T6 = T2, typename T7 = T3,
    std::enable_if_t<std::__is_transparent_v<std::__type_identity_t<T2>> &&
                     std::__is_transparent_v<std::__type_identity_t<T3>>> *T8 =
        nullptr>
typename std::unordered_multiset<T1, T2, T3, T4>::iterator
f120(std::unordered_multiset<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(!(std::is_same<T5, typename std::unordered_multiset<
                                  T1, T2, T3, T4>::key_type>::value))
{
  return a0.template find<T5>(a1);
}

template <
    typename T1, typename T2, typename T3, typename T4, typename T5,
    typename T6 = T2, typename T7 = T3,
    std::enable_if_t<std::__is_transparent_v<std::__type_identity_t<T2>> &&
                     std::__is_transparent_v<std::__type_identity_t<T3>>> *T8 =
        nullptr>
typename std::unordered_multiset<T1, T2, T3, T4>::const_iterator
f121(const std::unordered_multiset<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(!(std::is_same<T5, typename std::unordered_multiset<
                                  T1, T2, T3, T4>::key_type>::value))
{
  return a0.template find<T5>(a1);
}

template <
    typename T1, typename T2, typename T3, typename T4, typename T5,
    typename T6 = T2, typename T7 = T3,
    std::enable_if_t<std::__is_transparent_v<std::__type_identity_t<T2>> &&
                     std::__is_transparent_v<std::__type_identity_t<T3>>> *T8 =
        nullptr>
typename std::unordered_multiset<T1, T2, T3, T4>::size_type
f122(const std::unordered_multiset<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(!(std::is_same<T5, typename std::unordered_multiset<
                                  T1, T2, T3, T4>::key_type>::value))
{
  return a0.template count<T5>(a1);
}

template <
    typename T1, typename T2, typename T3, typename T4, typename T5,
    typename T6 = T2, typename T7 = T3,
    std::enable_if_t<std::__is_transparent_v<std::__type_identity_t<T2>> &&
                     std::__is_transparent_v<std::__type_identity_t<T3>>> *T8 =
        nullptr>
bool f123(const std::unordered_multiset<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(!(std::is_same<T5, typename std::unordered_multiset<
                                  T1, T2, T3, T4>::key_type>::value))
{
  return a0.template contains<T5>(a1);
}

template <
    typename T1, typename T2, typename T3, typename T4, typename T5,
    typename T6 = T2, typename T7 = T3,
    std::enable_if_t<std::__is_transparent_v<std::__type_identity_t<T2>> &&
                     std::__is_transparent_v<std::__type_identity_t<T3>>> *T8 =
        nullptr>
std::pair<typename std::unordered_multiset<T1, T2, T3, T4>::iterator,
          typename std::unordered_multiset<T1, T2, T3, T4>::iterator>
f124(std::unordered_multiset<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(!(std::is_same<T5, typename std::unordered_multiset<
                                  T1, T2, T3, T4>::key_type>::value))
{
  return a0.template equal_range<T5>(a1);
}

template <
    typename T1, typename T2, typename T3, typename T4, typename T5,
    typename T6 = T2, typename T7 = T3,
    std::enable_if_t<std::__is_transparent_v<std::__type_identity_t<T2>> &&
                     std::__is_transparent_v<std::__type_identity_t<T3>>> *T8 =
        nullptr>
std::pair<typename std::unordered_multiset<T1, T2, T3, T4>::const_iterator,
          typename std::unordered_multiset<T1, T2, T3, T4>::const_iterator>
f125(const std::unordered_multiset<T1, T2, T3, T4> &a0, const T5 &a1)
  requires(!(std::is_same<T5, typename std::unordered_multiset<
                                  T1, T2, T3, T4>::key_type>::value))
{
  return a0.template equal_range<T5>(a1);
}
