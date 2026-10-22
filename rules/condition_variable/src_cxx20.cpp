#include "../common/init.h"
#include "../common/traits.h"
#include <atomic>
#include <chrono>
#include <condition_variable>
#include <utility>

template <typename T1, typename T2>
bool f22(std::condition_variable_any &a0, T1 &a1, std::stop_token a2, T2 a3) {
  return a0.template wait<T1, T2>(a1, std::move(a2), a3);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f23(std::condition_variable_any &a0, T1 &a1, std::stop_token a2,
         const std::chrono::time_point<T2, T3> &a3, T4 a4) {
  return a0.template wait_until<T1, T2, T3, T4>(a1, std::move(a2), a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f24(std::condition_variable_any &a0, T1 &a1, std::stop_token a2,
         const std::chrono::duration<T2, T3> &a3, T4 a4) {
  return a0.template wait_for<T1, T2, T3, T4>(a1, std::move(a2), a3, a4);
}
