#include <algorithm>
#include <initializer_list>
#include <unordered_map>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4, typename T5>
bool f97(const std::unordered_map<T1, T2, T3, T4, T5> &a0,
         const typename std::unordered_map<T1, T2, T3, T4, T5>::key_type &a1) {
  return a0.contains(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::__enable_if_t<
    std::is_constructible<
        typename std::__umap_hashtable<T1, T2, T3, T4, T5>::value_type,
        T6 &&>::value,
    std::pair<typename std::__umap_hashtable<T1, T2, T3, T4, T5>::iterator,
              bool>>
f98(std::unordered_map<T1, T2, T3, T4, T5> &a0, T6 &&a1)
  requires(
      !(std::is_same<T6, typename std::unordered_map<T1, T2, T3, T4,
                                                     T5>::node_type>::value) &&
      !(std::is_same<T6, const typename std::unordered_map<
                             T1, T2, T3, T4, T5>::value_type &>::value) &&
      !(std::is_same<T6, typename std::unordered_map<T1, T2, T3, T4,
                                                     T5>::value_type>::value) &&
      !(std::is_same<T6, std::initializer_list<typename std::unordered_map<
                             T1, T2, T3, T4, T5>::value_type>>::value))
{
  return a0.template insert<T6>(std::forward<T6>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::__enable_if_t<std::is_constructible<typename std::__umap_hashtable<
                                             T1, T2, T3, T4, T5>::value_type,
                                         T6 &&>::value,
                   typename std::__umap_hashtable<T1, T2, T3, T4, T5>::iterator>
f99(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator a1, T6 &&a2)
  requires(
      !(std::is_same<T6, typename std::unordered_map<T1, T2, T3, T4,
                                                     T5>::node_type>::value) &&
      !(std::is_same<T6, const typename std::unordered_map<
                             T1, T2, T3, T4, T5>::value_type &>::value) &&
      !(std::is_same<T6, typename std::unordered_map<T1, T2, T3, T4,
                                                     T5>::value_type>::value))
{
  return a0.template insert<T6>(std::move(a1), std::forward<T6>(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
auto f100(std::unordered_map<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(!(std::is_same<T6, typename std::unordered_map<
                                  T1, T2, T3, T4, T5>::key_type>::value))
{
  return a0.template find<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
auto f101(const std::unordered_map<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(!(std::is_same<T6, typename std::unordered_map<
                                  T1, T2, T3, T4, T5>::key_type>::value))
{
  return a0.template find<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
auto f102(const std::unordered_map<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(!(std::is_same<T6, typename std::unordered_map<
                                  T1, T2, T3, T4, T5>::key_type>::value))
{
  return a0.template count<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
auto f103(const std::unordered_map<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(!(std::is_same<T6, typename std::unordered_map<
                                  T1, T2, T3, T4, T5>::key_type>::value))
{
  return a0.template contains<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
auto f104(std::unordered_map<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(!(std::is_same<T6, typename std::unordered_map<
                                  T1, T2, T3, T4, T5>::key_type>::value))
{
  return a0.template equal_range<T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
auto f105(const std::unordered_map<T1, T2, T3, T4, T5> &a0, const T6 &a1)
  requires(!(std::is_same<T6, typename std::unordered_map<
                                  T1, T2, T3, T4, T5>::key_type>::value))
{
  return a0.template equal_range<T6>(a1);
}
