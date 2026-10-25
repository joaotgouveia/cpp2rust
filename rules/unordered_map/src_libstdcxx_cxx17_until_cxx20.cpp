#include <algorithm>
#include <initializer_list>
#include <unordered_map>
#include <utility>

template <
    typename T1, typename T2, typename T3, typename T4, typename T5,
    typename T6,
    typename std::enable_if<
        (!(std::is_same<T6, typename std::unordered_map<
                                T1, T2, T3, T4, T5>::node_type>::value) &&
         !(std::is_same<T6, const typename std::unordered_map<
                                T1, T2, T3, T4, T5>::value_type &>::value) &&
         !(std::is_same<T6, typename std::unordered_map<
                                T1, T2, T3, T4, T5>::value_type>::value) &&
         !(std::is_same<T6, std::initializer_list<typename std::unordered_map<
                                T1, T2, T3, T4, T5>::value_type>>::value)),
        int>::type T7 = 0>
std::__enable_if_t<
    std::is_constructible<
        typename std::__umap_hashtable<T1, T2, T3, T4, T5>::value_type,
        T6 &&>::value,
    std::pair<typename std::__umap_hashtable<T1, T2, T3, T4, T5>::iterator,
              bool>>
f95(std::unordered_map<T1, T2, T3, T4, T5> &a0, T6 &&a1) {
  return a0.template insert<T6>(std::forward<T6>(a1));
}

template <
    typename T1, typename T2, typename T3, typename T4, typename T5,
    typename T6,
    typename std::enable_if<
        (!(std::is_same<T6, typename std::unordered_map<
                                T1, T2, T3, T4, T5>::node_type>::value) &&
         !(std::is_same<T6, const typename std::unordered_map<
                                T1, T2, T3, T4, T5>::value_type &>::value) &&
         !(std::is_same<T6, typename std::unordered_map<
                                T1, T2, T3, T4, T5>::value_type>::value)),
        int>::type T7 = 0>
std::__enable_if_t<std::is_constructible<typename std::__umap_hashtable<
                                             T1, T2, T3, T4, T5>::value_type,
                                         T6 &&>::value,
                   typename std::__umap_hashtable<T1, T2, T3, T4, T5>::iterator>
f96(std::unordered_map<T1, T2, T3, T4, T5> &a0,
    typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator a1,
    T6 &&a2) {
  return a0.template insert<T6>(
      std::forward<
          typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator>(a1),
      std::forward<T6>(a2));
}
