#include <ios>
#include <iosfwd>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2>
using t11 = typename std::ostreambuf_iterator<T1, T2>::difference_type;

template <typename T1, typename T2> std::ostreambuf_iterator<T1, T2> f14() {
  return std::ostreambuf_iterator<T1, T2>();
}
