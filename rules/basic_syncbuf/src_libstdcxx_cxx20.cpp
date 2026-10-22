#include "../common/init.h"
#include "../common/traits.h"
#include <ios>
#include <iosfwd>
#include <syncstream>
#include <utility>

template <typename T1, typename T2, typename T3>
std::basic_syncbuf<T1, T2, T3>
f24(typename std::basic_syncbuf<T1, T2, T3>::streambuf_type *a0,
    const typename std::basic_syncbuf<T1, T2, T3>::allocator_type &a1) {
  return std::basic_syncbuf<T1, T2, T3>(a0, a1);
}
