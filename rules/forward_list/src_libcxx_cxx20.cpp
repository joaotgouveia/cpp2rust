#include <forward_list>
#include <initializer_list>
#include <utility>

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::size_type
f101(std::forward_list<T1, T2> &a0,
     const typename std::forward_list<T1, T2>::value_type &a1) {
  return a0.remove(a1);
}
