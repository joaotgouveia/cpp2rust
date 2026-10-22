#include "../common/init.h"
#include "../common/traits.h"
#include <deque>
#include <initializer_list>
#include <utility>

template <typename T1, typename T2>
auto f92(const std::deque<T1, T2> &a0, const std::deque<T1, T2> &a1) {
  return a0 <=> a1;
}
