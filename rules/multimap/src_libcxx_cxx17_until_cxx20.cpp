#include <initializer_list>
#include <map>
#include <utility>

template <
    typename T1, typename T2, typename T3, typename T4, typename T5,
    std::__enable_if_t<
        std::is_constructible<std::pair<const T1, T2>, T5>::value, int>
        T6 = 0,
    typename std::enable_if<
        (!(std::is_same<
             T5, typename std::multimap<T1, T2, T3, T4>::value_type>::value) &&
         !(std::is_same<T5, std::initializer_list<typename std::multimap<
                                T1, T2, T3, T4>::value_type>>::value) &&
         !(std::is_same<T5, const typename std::multimap<
                                T1, T2, T3, T4>::value_type &>::value) &&
         !(std::is_same<
             T5, typename std::multimap<T1, T2, T3, T4>::node_type>::value)),
        int>::type T7 = 0>
typename std::multimap<T1, T2, T3, T4>::iterator
f109(std::multimap<T1, T2, T3, T4> &a0, T5 &&a1) {
  return a0.template insert<T5>(std::forward<T5>(a1));
}

template <
    typename T1, typename T2, typename T3, typename T4, typename T5,
    std::__enable_if_t<
        std::is_constructible<std::pair<const T1, T2>, T5>::value, int>
        T6 = 0,
    typename std::enable_if<
        (!(std::is_same<
             T5, typename std::multimap<T1, T2, T3, T4>::value_type>::value) &&
         !(std::is_same<T5, const typename std::multimap<
                                T1, T2, T3, T4>::value_type &>::value) &&
         !(std::is_same<
             T5, typename std::multimap<T1, T2, T3, T4>::node_type>::value)),
        int>::type T7 = 0>
typename std::multimap<T1, T2, T3, T4>::iterator
f110(std::multimap<T1, T2, T3, T4> &a0,
     typename std::multimap<T1, T2, T3, T4>::const_iterator a1, T5 &&a2) {
  return a0.template insert<T5>(std::move(a1), std::forward<T5>(a2));
}
