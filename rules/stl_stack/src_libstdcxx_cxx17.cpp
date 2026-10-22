#include "../common/init.h"
#include "../common/traits.h"
#include <stack>
#include <utility>

template <typename T1, typename T2>
typename std::enable_if<std::is_swappable<T2>::value>::type
f8(std::stack<T1, T2> &a0, std::stack<T1, T2> &a1) {
  return std::swap<T1, T2>(a0, a1);
}
