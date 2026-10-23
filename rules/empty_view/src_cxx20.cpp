#include <algorithm>
#include <cstddef>
#include <ranges>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1> using t1 = std::ranges::empty_view<T1>;

template <typename T1> T1 *f1() { return std::ranges::empty_view<T1>::begin(); }

template <typename T1> T1 *f2() { return std::ranges::empty_view<T1>::end(); }

template <typename T1> T1 *f3() { return std::ranges::empty_view<T1>::data(); }

template <typename T1> std::size_t f4() {
  return std::ranges::empty_view<T1>::size();
}

template <typename T1> bool f5() {
  return std::ranges::empty_view<T1>::empty();
}
