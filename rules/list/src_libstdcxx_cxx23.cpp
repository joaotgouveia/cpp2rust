#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <initializer_list>
#include <list>
#include <utility>

template <typename T1, typename T2, typename T3>
std::list<T1, T2> f86(std::from_range_t a0, T3 &&a1, const T2 &a2) {
  return std::list<T1, T2>(std::move(a0), std::forward<T3>(a1), a2);
}
