#include <algorithm>
#include <utility>
#include <vector>

template <typename T1, typename T2> T2 f1(T1 a0, T1 a1, T2 a2) {
  return std::uninitialized_copy(std::move(a0), std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3> T3 f2(T1 a0, T2 a1, T3 a2) {
  return std::uninitialized_copy_n(std::move(a0), std::move(a1), std::move(a2));
}

template <typename T1> void f3(T1 a0, T1 a1) {
  return std::uninitialized_default_construct(std::move(a0), std::move(a1));
}

template <typename T1, typename T2> T1 f4(T1 a0, T2 a1) {
  return std::uninitialized_default_construct_n(std::move(a0), std::move(a1));
}

template <typename T1, typename T2> void f5(T1 a0, T1 a1, const T2 &a2) {
  return std::uninitialized_fill(std::move(a0), std::move(a1), a2);
}

template <typename T1, typename T2, typename T3>
T1 f6(T1 a0, T2 a1, const T3 &a2) {
  return std::uninitialized_fill_n(std::move(a0), std::move(a1), a2);
}

template <typename T1, typename T2> T2 f7(T1 a0, T1 a1, T2 a2) {
  return std::uninitialized_move(std::move(a0), std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3>
std::pair<T1, T3> f8(T1 a0, T2 a1, T3 a2) {
  return std::uninitialized_move_n(std::move(a0), std::move(a1), std::move(a2));
}

template <typename T1> void f9(T1 a0, T1 a1) {
  return std::uninitialized_value_construct(std::move(a0), std::move(a1));
}

template <typename T1, typename T2> T1 f10(T1 a0, T2 a1) {
  return std::uninitialized_value_construct_n(std::move(a0), std::move(a1));
}
