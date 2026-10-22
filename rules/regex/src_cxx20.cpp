#include "../common/init.h"
#include "../common/traits.h"
#include <barrier>
#include <ostream>
#include <regex>
#include <string>
#include <utility>

template <typename T1>
auto f66(const std::sub_match<T1> &a0,
         const typename std::iterator_traits<T1>::value_type &a1) {
  return a0 <=> a1;
}

template <typename T1>
auto f67(const std::sub_match<T1> &a0,
         const typename std::iterator_traits<T1>::value_type *a1) {
  return a0 <=> a1;
}

template <typename T1>
auto f68(const std::sub_match<T1> &a0, const std::sub_match<T1> &a1) {
  return a0 <=> a1;
}

template <typename T1, typename T2, typename T3>
auto f69(const std::sub_match<T1> &a0,
         const std::basic_string<typename std::iterator_traits<T1>::value_type,
                                 T2, T3> &a1) {
  return a0 <=> a1;
}
