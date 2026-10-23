#include <algorithm>
#include <ios>
#include <iosfwd>
#include <istream>
#include <streambuf>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2>
std::basic_iostream<T1, T2> &f99(std::basic_iostream<T1, T2> &a0,
                                 const volatile void *a1) {
  return a0.operator<<(a1);
}
