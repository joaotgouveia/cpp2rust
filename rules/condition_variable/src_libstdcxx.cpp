#include <chrono>
#include <condition_variable>
#include <stdlib.h>
#include <utility>

template <typename T1>
std::cv_status
f26(std::condition_variable &a0, std::unique_lock<std::mutex> &a1,
    const std::chrono::time_point<std::chrono::steady_clock, T1> &a2) {
  return a0.template wait_until<T1>(a1, a2);
}

template <typename T1>
std::cv_status
f27(std::condition_variable &a0, std::unique_lock<std::mutex> &a1,
    const std::chrono::time_point<std::chrono::system_clock, T1> &a2) {
  return a0.template wait_until<T1>(a1, a2);
}
