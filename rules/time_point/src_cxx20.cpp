#include "../common/init.h"
#include "../common/traits.h"
#include <chrono>
#include <utility>

template <typename T1, typename T2>
std::chrono::time_point<T1, T2> &f9(std::chrono::time_point<T1, T2> &a0) {
  return a0.operator++();
}

template <typename T1, typename T2>
std::chrono::time_point<T1, T2> f10(std::chrono::time_point<T1, T2> &a0,
                                    int a1) {
  return a0.operator++(a1);
}

template <typename T1, typename T2>
std::chrono::time_point<T1, T2> &f11(std::chrono::time_point<T1, T2> &a0) {
  return a0.operator--();
}

template <typename T1, typename T2>
std::chrono::time_point<T1, T2> f12(std::chrono::time_point<T1, T2> &a0,
                                    int a1) {
  return a0.operator--(a1);
}
