#include "../common/init.h"
#include "../common/traits.h"
#include <mutex>
#include <utility>

template <typename T1, typename... T2>
void f1(std::once_flag &a0, T1 &&a1, T2 &&...a2) {
  return std::call_once<T1, T2...>(a0, std::forward<T1>(a1),
                                   std::forward<T2>(a2)...);
}

using t1 = std::once_flag;

std::once_flag f2() { return {}; }
