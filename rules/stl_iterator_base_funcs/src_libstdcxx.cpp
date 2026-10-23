#include <algorithm>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2> void f1(T1 &a0, T2 a1) {
  return std::advance<T1, T2>(a0, a1);
}

template <typename T1>
typename std::iterator_traits<T1>::difference_type f2(T1 a0, T1 a1) {
  return std::distance<T1>(a0, a1);
}

template <typename T1>
T1 f3(T1 a0, typename std::iterator_traits<T1>::difference_type a1) {
  return std::next<T1>(a0, a1);
}

template <typename T1>
T1 f4(T1 a0, typename std::iterator_traits<T1>::difference_type a1) {
  return std::prev<T1>(a0, a1);
}
