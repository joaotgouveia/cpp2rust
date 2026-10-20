#include "../common/init.h"
#include "../common/traits.h"
#include <list>
#include <utility>

template <typename T1, typename T2>
auto f8(const std::list<T1, T2> &a0, const std::list<T1, T2> &a1) {
  return a0 <=> a1;
}
