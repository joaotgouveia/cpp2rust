#include <compare>
#include <iterator>
#include <utility>

template <typename T1>
using t7 = typename std::__wrap_iter<T1>::iterator_concept;

template <typename T1, typename T2>
std::strong_ordering f28(const std::__wrap_iter<T1> &a0,
                         const std::__wrap_iter<T2> &a1) {
  return operator<=>(a0, a1);
}
