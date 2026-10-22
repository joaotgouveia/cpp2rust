#include "../common/init.h"
#include "../common/traits.h"
#include <utility>
#include <vector>

template <typename T1>
typename std::enable_if<std::is_move_constructible<T1>::value &&
                            std::is_move_assignable<T1>::value,
                        void>::type
f1(T1 &a0, T1 &a1) {
  return std::swap<T1>(a0, a1);
}

template <typename T1, typename T2>
void f2(std::vector<T1, T2> &a0, std::vector<T1, T2> &a1) {
  return std::swap<T1, T2>(a0, a1);
}
