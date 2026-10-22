#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <bitset>
#include <cstddef>
#include <ios>
#include <memory>
#include <ostream>
#include <streambuf>
#include <string>
#include <string_view>
#include <system_error>
#include <utility>

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f53(std::basic_ostream<T1, T2> &a0,
                                std::nullptr_t a1) {
  return a0.operator<<(a1);
}
