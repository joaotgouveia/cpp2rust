#include <atomic>
#include <chrono>
#include <condition_variable>
#include <ios>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

using t1 = std::condition_variable;

using t2 = std::condition_variable::native_handle_type;

std::condition_variable f1() { return {}; }

void f2(std::condition_variable &a0) { return a0.notify_one(); }

void f3(std::condition_variable &a0) { return a0.notify_all(); }

void f4(std::condition_variable &a0, std::unique_lock<std::mutex> &a1) {
  return a0.wait(a1);
}

template <typename T1>
void f5(std::condition_variable &a0, std::unique_lock<std::mutex> &a1, T1 a2) {
  return a0.template wait<T1>(a1, a2);
}

template <typename T1, typename T2>
std::cv_status f6(std::condition_variable &a0, std::unique_lock<std::mutex> &a1,
                  const std::chrono::time_point<T1, T2> &a2) {
  return a0.template wait_until<T1, T2>(a1, a2);
}

template <typename T1, typename T2, typename T3>
bool f7(std::condition_variable &a0, std::unique_lock<std::mutex> &a1,
        const std::chrono::time_point<T1, T2> &a2, T3 a3) {
  return a0.template wait_until<T1, T2, T3>(a1, a2, a3);
}

template <typename T1, typename T2>
std::cv_status f8(std::condition_variable &a0, std::unique_lock<std::mutex> &a1,
                  const std::chrono::duration<T1, T2> &a2) {
  return a0.template wait_for<T1, T2>(a1, a2);
}

template <typename T1, typename T2, typename T3>
bool f9(std::condition_variable &a0, std::unique_lock<std::mutex> &a1,
        const std::chrono::duration<T1, T2> &a2, T3 a3) {
  return a0.template wait_for<T1, T2, T3>(a1, a2, a3);
}

using t3 = std::condition_variable_any;

std::condition_variable_any f10() { return {}; }

void f11(std::condition_variable_any &a0) { return a0.notify_one(); }

void f12(std::condition_variable_any &a0) { return a0.notify_all(); }

template <typename T1> void f13(std::condition_variable_any &a0, T1 &a1) {
  return a0.template wait<T1>(a1);
}

template <typename T1, typename T2>
void f14(std::condition_variable_any &a0, T1 &a1, T2 a2) {
  return a0.template wait<T1, T2>(a1, a2);
}

template <typename T1, typename T2, typename T3>
std::cv_status f15(std::condition_variable_any &a0, T1 &a1,
                   const std::chrono::time_point<T2, T3> &a2) {
  return a0.template wait_until<T1, T2, T3>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f16(std::condition_variable_any &a0, T1 &a1,
         const std::chrono::time_point<T2, T3> &a2, T4 a3) {
  return a0.template wait_until<T1, T2, T3, T4>(a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
std::cv_status f17(std::condition_variable_any &a0, T1 &a1,
                   const std::chrono::duration<T2, T3> &a2) {
  return a0.template wait_for<T1, T2, T3>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f18(std::condition_variable_any &a0, T1 &a1,
         const std::chrono::duration<T2, T3> &a2, T4 a3) {
  return a0.template wait_for<T1, T2, T3, T4>(a1, a2, a3);
}

using t4 = std::cv_status;

std::cv_status f19() { return std::cv_status::no_timeout; }

std::cv_status f20() { return std::cv_status::timeout; }

void f21(std::condition_variable &a0, std::unique_lock<std::mutex> a1) {
  return std::notify_all_at_thread_exit(a0, std::move(a1));
}
