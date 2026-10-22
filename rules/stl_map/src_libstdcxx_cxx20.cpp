#include "../common/init.h"
#include "../common/traits.h"
#include <map>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
auto f8(const std::map<T1, T2, T3, T4> &a0,
        const std::map<T1, T2, T3, T4> &a1) {
  return a0 <=> a1;
}
