#include "../common/init.h"
#include "../common/traits.h"
#include <memory>
#include <utility>

template <typename T1> void f1(T1 a0, T1 a1) {
  return std::destroy<T1>(a0, a1);
}

template <typename T1, typename T2> T1 f2(T1 a0, T2 a1) {
  return std::destroy_n<T1, T2>(a0, a1);
}
