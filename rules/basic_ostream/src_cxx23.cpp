#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <barrier>
#include <bitset>
#include <cstddef>
#include <ios>
#include <memory>
#include <ostream>
#include <streambuf>
#include <string_view>
#include <system_error>
#include <utility>

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f54(std::basic_ostream<T1, T2> &a0,
                                const volatile void *a1) {
  return a0.operator<<(a1);
}
