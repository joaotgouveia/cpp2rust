#include "../common/init.h"
#include "../common/traits.h"
#include <iterator>
#include <utility>

template <typename T1>
typename std::iterator_traits<T1>::difference_type f1(T1 a0, T1 a1) {
  return std::distance<T1>(a0, a1);
}
