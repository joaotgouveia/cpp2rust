#include "../common/init.h"
#include "../common/traits.h"
#include <memory>
#include <utility>

template <typename T1> void f5(T1 a0, T1 a1) {
  return std::uninitialized_default_construct<T1>(a0, a1);
}

template <typename T1, typename T2> T1 f6(T1 a0, T2 a1) {
  return std::uninitialized_default_construct_n<T1, T2>(a0, a1);
}

template <typename T1, typename T2> T2 f7(T1 a0, T1 a1, T2 a2) {
  return std::uninitialized_move<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3>
std::pair<T1, T3> f8(T1 a0, T2 a1, T3 a2) {
  return std::uninitialized_move_n<T1, T2, T3>(a0, a1, a2);
}

template <typename T1> void f9(T1 a0, T1 a1) {
  return std::uninitialized_value_construct<T1>(a0, a1);
}

template <typename T1, typename T2> T1 f10(T1 a0, T2 a1) {
  return std::uninitialized_value_construct_n<T1, T2>(a0, a1);
}
