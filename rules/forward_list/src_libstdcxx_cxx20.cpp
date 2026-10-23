#include <forward_list>
#include <initializer_list>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::size_type f81(std::forward_list<T1, T2> &a0,
                                                  const T1 &a1) {
  return a0.remove(a1);
}
