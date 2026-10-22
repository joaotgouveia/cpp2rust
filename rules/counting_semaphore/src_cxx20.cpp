#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <chrono>
#include <cstddef>
#include <semaphore>
#include <utility>

template <std::ptrdiff_t T1> using t1 = std::counting_semaphore<T1>;

template <std::ptrdiff_t T1> std::counting_semaphore<T1> f1(std::ptrdiff_t a0) {
  return std::counting_semaphore<T1>(a0);
}

template <std::ptrdiff_t T1> std::ptrdiff_t f2() {
  return std::counting_semaphore<T1>::max();
}

template <std::ptrdiff_t T1>
void f3(std::counting_semaphore<T1> &a0, std::ptrdiff_t a1) {
  return a0.release(a1);
}

template <std::ptrdiff_t T1> void f4(std::counting_semaphore<T1> &a0) {
  return a0.acquire();
}

template <std::ptrdiff_t T1> bool f5(std::counting_semaphore<T1> &a0) {
  return a0.try_acquire();
}

template <std::ptrdiff_t T1, typename T2, typename T3>
bool f6(std::counting_semaphore<T1> &a0,
        const std::chrono::duration<T2, T3> &a1) {
  return a0.template try_acquire_for<T2, T3>(a1);
}

template <std::ptrdiff_t T1, typename T2, typename T3>
bool f7(std::counting_semaphore<T1> &a0,
        const std::chrono::time_point<T2, T3> &a1) {
  return a0.template try_acquire_until<T2, T3>(a1);
}
