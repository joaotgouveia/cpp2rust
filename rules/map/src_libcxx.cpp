#include <initializer_list>
#include <map>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
bool f103(const std::map<T1, T2, T3, T4> &a0,
          const std::map<T1, T2, T3, T4> &a1) {
  return std::operator== <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f104(const std::multimap<T1, T2, T3, T4> &a0,
          const std::multimap<T1, T2, T3, T4> &a1) {
  return std::operator== <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
void f105(std::map<T1, T2, T3, T4> &a0, std::map<T1, T2, T3, T4> &a1) {
  return std::swap<T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
void f106(std::multimap<T1, T2, T3, T4> &a0,
          std::multimap<T1, T2, T3, T4> &a1) {
  return std::swap<T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::map<T1, T2, T3, T4>
f107(const typename std::map<T1, T2, T3, T4>::key_compare &a0) {
  return std::map<T1, T2, T3, T4>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::map<T1, T2, T3, T4>
f108(const typename std::map<T1, T2, T3, T4>::key_compare &a0,
     const typename std::map<T1, T2, T3, T4>::allocator_type &a1) {
  return std::map<T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::map<T1, T2, T3, T4>
f109(std::initializer_list<typename std::map<T1, T2, T3, T4>::value_type> a0,
     const typename std::map<T1, T2, T3, T4>::key_compare &a1) {
  return std::map<T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::map<T1, T2, T3, T4>
f110(std::initializer_list<typename std::map<T1, T2, T3, T4>::value_type> a0,
     const typename std::map<T1, T2, T3, T4>::key_compare &a1,
     const typename std::map<T1, T2, T3, T4>::allocator_type &a2) {
  return std::map<T1, T2, T3, T4>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::map<T1, T2, T3, T4>
f111(T5 a0, T5 a1, const typename std::map<T1, T2, T3, T4>::key_compare &a2) {
  return std::map<T1, T2, T3, T4>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::map<T1, T2, T3, T4>
f112(T5 a0, T5 a1, const typename std::map<T1, T2, T3, T4>::key_compare &a2,
     const typename std::map<T1, T2, T3, T4>::allocator_type &a3) {
  return std::map<T1, T2, T3, T4>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename std::enable_if<
              std::is_constructible<std::pair<const T1, T2>, T5>::value,
              int>::type T6 = 0>
std::pair<typename std::map<T1, T2, T3, T4>::iterator, bool>
f113(std::map<T1, T2, T3, T4> &a0, T5 &&a1) {
  return a0.template insert<T5>(std::forward<T5>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename std::enable_if<
              std::is_constructible<std::pair<const T1, T2>, T5>::value,
              int>::type T6 = 0>
typename std::map<T1, T2, T3, T4>::iterator
f114(std::map<T1, T2, T3, T4> &a0,
     typename std::map<T1, T2, T3, T4>::const_iterator a1, T5 &&a2) {
  return a0.template insert<T5>(a1, std::forward<T5>(a2));
}

template <typename T1, typename T2, typename T3, typename T4>
using t28 = typename std::map<T1, T2, T3, T4>::value_type::first_type;

template <typename T1, typename T2, typename T3, typename T4>
using t29 = typename std::map<T1, T2, T3, T4>::value_type::second_type;

template <typename T1, typename T2, typename T3, typename T4>
using t30 =
    typename std::map<T1, T2, T3, T4>::reverse_iterator::iterator_category;

template <typename T1, typename T2, typename T3, typename T4>
using t31 = typename std::map<T1, T2, T3, T4>::reverse_iterator::pointer;

template <typename T1, typename T2, typename T3, typename T4>
using t32 = typename std::map<T1, T2, T3, T4>::reverse_iterator::value_type;

template <typename T1, typename T2, typename T3, typename T4>
using t33 =
    typename std::map<T1, T2, T3, T4>::reverse_iterator::difference_type;

template <typename T1, typename T2, typename T3, typename T4>
using t34 = typename std::map<T1, T2, T3, T4>::reverse_iterator::reference;

template <typename T1, typename T2, typename T3, typename T4>
using t35 = typename std::map<T1, T2, T3,
                              T4>::const_reverse_iterator::iterator_category;

template <typename T1, typename T2, typename T3, typename T4>
using t36 = typename std::map<T1, T2, T3, T4>::const_reverse_iterator::pointer;

template <typename T1, typename T2, typename T3, typename T4>
using t37 =
    typename std::map<T1, T2, T3, T4>::const_reverse_iterator::value_type;

template <typename T1, typename T2, typename T3, typename T4>
using t38 =
    typename std::map<T1, T2, T3, T4>::const_reverse_iterator::difference_type;

template <typename T1, typename T2, typename T3, typename T4>
using t39 =
    typename std::map<T1, T2, T3, T4>::const_reverse_iterator::reference;
