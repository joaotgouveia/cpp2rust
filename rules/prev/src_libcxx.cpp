#include "../common/init.h"
#include "../common/traits.h"
#include <iterator>
#include <utility>

template <typename T1> T1 f1(T1 a0) { return std::prev<T1>(a0); }

template <typename T1>
T1 f2(T1 a0, typename std::iterator_traits<T1>::difference_type a1) {
  return std::prev<T1>(a0, a1);
}
