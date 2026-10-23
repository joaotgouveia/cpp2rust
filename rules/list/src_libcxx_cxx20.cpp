#include <initializer_list>
#include <list>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2>
typename std::list<T1, T2>::size_type
f104(std::list<T1, T2> &a0, const typename std::list<T1, T2>::value_type &a1) {
  return a0.remove(a1);
}

template <typename T1, typename T2>
auto f105(const std::list<T1, T2> &a0, const std::list<T1, T2> &a1) {
  return a0 <=> a1;
}
