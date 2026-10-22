#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <ios>
#include <ostream>
#include <streambuf>
#include <string>
#include <utility>

template <typename T1, typename T2>
std::basic_ostream<T1, T2> f61(std::basic_streambuf<T1, T2> *a0) {
  return std::basic_ostream<T1, T2>(a0);
}
