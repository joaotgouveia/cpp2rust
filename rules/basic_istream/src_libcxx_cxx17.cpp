#include "../common/init.h"
#include "../common/traits.h"
#include <ios>
#include <istream>
#include <ostream>
#include <streambuf>
#include <utility>

template <typename T1, typename T2>
std::basic_istream<T1, T2>
f84(std::basic_streambuf<typename std::basic_istream<T1, T2>::char_type,
                         typename std::basic_istream<T1, T2>::traits_type>
        *a0) {
  return std::basic_istream<T1, T2>(a0);
}
