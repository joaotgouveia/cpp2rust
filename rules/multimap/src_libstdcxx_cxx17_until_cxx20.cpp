#include <algorithm>
#include <initializer_list>
#include <map>
#include <utility>

template <
    typename T1, typename T2, typename T3, typename T4, typename T5,
    typename std::enable_if<
        (!(std::is_same<T5, const typename std::multimap<
                                T1, T2, T3, T4>::value_type &>::value) &&
         !(std::is_same<
             T5, typename std::multimap<T1, T2, T3, T4>::value_type>::value) &&
         !(std::is_same<T5, std::initializer_list<typename std::multimap<
                                T1, T2, T3, T4>::value_type>>::value) &&
         !(std::is_same<
             T5, typename std::multimap<T1, T2, T3, T4>::node_type>::value)),
        int>::type T6 = 0>
std::__enable_if_t<std::is_constructible<std::pair<const T1, T2>, T5>::value,
                   typename std::_Rb_tree<
                       T1, std::pair<const T1, T2>,
                       std::_Select1st<std::pair<const T1, T2>>, T3,
                       typename __gnu_cxx::__alloc_traits<T4>::template rebind<
                           std::pair<const T1, T2>>::other>::iterator>
f78(std::multimap<T1, T2, T3, T4> &a0, T5 &&a1) {
  return a0.template insert<T5>(std::forward<T5>(a1));
}

template <
    typename T1, typename T2, typename T3, typename T4, typename T5,
    typename std::enable_if<
        (!(std::is_same<T5, const typename std::multimap<
                                T1, T2, T3, T4>::value_type &>::value) &&
         !(std::is_same<
             T5, typename std::multimap<T1, T2, T3, T4>::value_type>::value) &&
         !(std::is_same<
             T5, typename std::multimap<T1, T2, T3, T4>::node_type>::value)),
        int>::type T6 = 0>
std::__enable_if_t<std::is_constructible<std::pair<const T1, T2>, T5 &&>::value,
                   typename std::_Rb_tree<
                       T1, std::pair<const T1, T2>,
                       std::_Select1st<std::pair<const T1, T2>>, T3,
                       typename __gnu_cxx::__alloc_traits<T4>::template rebind<
                           std::pair<const T1, T2>>::other>::iterator>
f79(std::multimap<T1, T2, T3, T4> &a0,
    typename std::multimap<T1, T2, T3, T4>::const_iterator a1, T5 &&a2) {
  return a0.template insert<T5>(a1, std::forward<T5>(a2));
}
