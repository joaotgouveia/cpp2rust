#include <algorithm>
#include <ios>
#include <iosfwd>
#include <iterator>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2>
std::istreambuf_iterator<T1, T2> f9(std::default_sentinel_t a0) {
  return std::istreambuf_iterator<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
bool f10(const std::istreambuf_iterator<T1, T2> &a0,
         std::default_sentinel_t a1) {
  return a0 == std::move(a1);
}
