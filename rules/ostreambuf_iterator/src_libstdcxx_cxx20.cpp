#include <ios>
#include <iosfwd>
#include <utility>

template <typename T1, typename T2>
using t6 = typename std::ostreambuf_iterator<T1, T2>::difference_type;

template <typename T1, typename T2> std::ostreambuf_iterator<T1, T2> f8() {
  return std::ostreambuf_iterator<T1, T2>();
}
