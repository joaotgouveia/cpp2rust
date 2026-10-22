#include "../common/init.h"
#include "../common/traits.h"
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
std::basic_ostream<T1, T2>
f87(std::basic_streambuf<typename std::basic_ostream<T1, T2>::char_type,
                         typename std::basic_ostream<T1, T2>::traits_type>
        *a0) {
  return std::basic_ostream<T1, T2>(a0);
}
