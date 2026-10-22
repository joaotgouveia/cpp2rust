#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <initializer_list>
#include <iosfwd>
#include <string>
#include <string_view>
#include <utility>

template <typename T1, typename T2, typename T3>
bool f68(const std::basic_string<T1, T2, T3> &a0,
         std::basic_string_view<T1, T2> a1) {
  return a0.contains(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
void f69(std::basic_string<T1, T2, T3> &a0,
         typename std::basic_string<T1, T2, T3>::size_type a1, T4 a2) {
  return a0.template resize_and_overwrite<T4>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<T1, T2, T3> &f70(std::basic_string<T1, T2, T3> &a0, T4 &&a1) {
  return a0.template append_range<T4>(std::forward<T4>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<T1, T2, T3> &f71(std::basic_string<T1, T2, T3> &a0, T4 &&a1) {
  return a0.template assign_range<T4>(std::forward<T4>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::basic_string<T1, T2, T3>::iterator
f72(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::const_iterator a1, T4 &&a2) {
  return a0.template insert_range<T4>(a1, std::forward<T4>(a2));
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<T1, T2, T3> &
f73(std::basic_string<T1, T2, T3> &a0,
    typename std::basic_string<T1, T2, T3>::const_iterator a1,
    typename std::basic_string<T1, T2, T3>::const_iterator a2, T4 &&a3) {
  return a0.template replace_with_range<T4>(a1, a2, std::forward<T4>(a3));
}
