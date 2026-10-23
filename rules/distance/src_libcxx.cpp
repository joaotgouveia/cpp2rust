#include <iterator>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1>
typename std::iterator_traits<T1>::difference_type f1(T1 a0, T1 a1) {
  return std::distance<T1>(a0, a1);
}
