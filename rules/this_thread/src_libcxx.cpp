#include <chrono>
#include <thread>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

void f1(const std::chrono::nanoseconds &a0) {
  return std::this_thread::sleep_for(a0);
}

template <typename T1, typename T2>
void f2(const std::chrono::duration<T1, T2> &a0) {
  return std::this_thread::sleep_for<T1, T2>(a0);
}

template <typename T1>
void f3(const std::chrono::time_point<std::chrono::steady_clock, T1> &a0) {
  return std::this_thread::sleep_until<T1>(a0);
}

template <typename T1, typename T2>
void f4(const std::chrono::time_point<T1, T2> &a0) {
  return std::this_thread::sleep_until<T1, T2>(a0);
}

void f5() { return std::this_thread::yield(); }
