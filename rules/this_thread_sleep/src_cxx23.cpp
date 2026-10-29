#include <chrono>
#include <semaphore>
#include <utility>

template <typename T1, typename T2>
void f1(const std::chrono::duration<T1, T2> &a0) {
  return std::this_thread::sleep_for<T1, T2>(a0);
}

template <typename T1, typename T2>
void f2(const std::chrono::time_point<T1, T2> &a0) {
  return std::this_thread::sleep_until<T1, T2>(a0);
}
