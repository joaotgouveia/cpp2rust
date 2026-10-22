#include "../common/init.h"
#include "../common/traits.h"
#include <ios>
#include <utility>

template <typename T1, typename T2>
bool f5(const std::istreambuf_iterator<T1, T2> &a0,
        const std::istreambuf_iterator<T1, T2> &a1) {
  return a0 != a1;
}
