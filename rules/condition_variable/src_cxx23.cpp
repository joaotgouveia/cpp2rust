#include <atomic>
#include <chrono>
#include <condition_variable>
#include <stdlib.h>
#include <utility>

using t1 = std::condition_variable;

using t2 = std::condition_variable::native_handle_type;

std::condition_variable f1() { return {}; }

void f2(std::condition_variable &a0) { return a0.notify_one(); }

void f3(std::condition_variable &a0) { return a0.notify_all(); }

void f4(std::condition_variable &a0, std::unique_lock<std::mutex> &a1) {
  return a0.wait(a1);
}

pthread_cond_t *f5(std::condition_variable &a0) { return a0.native_handle(); }

template <typename T1>
void f6(std::condition_variable &a0, std::unique_lock<std::mutex> &a1, T1 a2) {
  return a0.template wait<T1>(a1, std::move(a2));
}

template <typename T1>
std::cv_status
f7(std::condition_variable &a0, std::unique_lock<std::mutex> &a1,
   const std::chrono::time_point<std::chrono::steady_clock, T1> &a2) {
  return a0.template wait_until<T1>(a1, a2);
}

template <typename T1>
std::cv_status
f8(std::condition_variable &a0, std::unique_lock<std::mutex> &a1,
   const std::chrono::time_point<std::chrono::system_clock, T1> &a2) {
  return a0.template wait_until<T1>(a1, a2);
}

template <typename T1, typename T2>
std::cv_status f9(std::condition_variable &a0, std::unique_lock<std::mutex> &a1,
                  const std::chrono::time_point<T1, T2> &a2) {
  return a0.template wait_until<T1, T2>(a1, a2);
}

template <typename T1, typename T2, typename T3>
bool f10(std::condition_variable &a0, std::unique_lock<std::mutex> &a1,
         const std::chrono::time_point<T1, T2> &a2, T3 a3) {
  return a0.template wait_until<T1, T2, T3>(a1, a2, std::move(a3));
}

template <typename T1, typename T2>
std::cv_status f11(std::condition_variable &a0,
                   std::unique_lock<std::mutex> &a1,
                   const std::chrono::duration<T1, T2> &a2) {
  return a0.template wait_for<T1, T2>(a1, a2);
}

template <typename T1, typename T2, typename T3>
bool f12(std::condition_variable &a0, std::unique_lock<std::mutex> &a1,
         const std::chrono::duration<T1, T2> &a2, T3 a3) {
  return a0.template wait_for<T1, T2, T3>(a1, a2, std::move(a3));
}

using t3 = std::condition_variable_any;

std::condition_variable_any f13() { return {}; }

void f14(std::condition_variable_any &a0) { return a0.notify_one(); }

void f15(std::condition_variable_any &a0) { return a0.notify_all(); }

template <typename T1> void f16(std::condition_variable_any &a0, T1 &a1) {
  return a0.template wait<T1>(a1);
}

template <typename T1, typename T2>
void f17(std::condition_variable_any &a0, T1 &a1, T2 a2) {
  return a0.template wait<T1, T2>(a1, std::move(a2));
}

template <typename T1, typename T2, typename T3>
std::cv_status f18(std::condition_variable_any &a0, T1 &a1,
                   const std::chrono::time_point<T2, T3> &a2) {
  return a0.template wait_until<T1, T2, T3>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f19(std::condition_variable_any &a0, T1 &a1,
         const std::chrono::time_point<T2, T3> &a2, T4 a3) {
  return a0.template wait_until<T1, T2, T3, T4>(a1, a2, std::move(a3));
}

template <typename T1, typename T2, typename T3>
std::cv_status f20(std::condition_variable_any &a0, T1 &a1,
                   const std::chrono::duration<T2, T3> &a2) {
  return a0.template wait_for<T1, T2, T3>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f21(std::condition_variable_any &a0, T1 &a1,
         const std::chrono::duration<T2, T3> &a2, T4 a3) {
  return a0.template wait_for<T1, T2, T3, T4>(a1, a2, std::move(a3));
}

template <typename T1, typename T2>
bool f22(std::condition_variable_any &a0, T1 &a1, std::stop_token a2, T2 a3) {
  return a0.template wait<T1, T2>(a1, std::move(a2), std::move(a3));
}

template <typename T1, typename T2, typename T3, typename T4>
bool f23(std::condition_variable_any &a0, T1 &a1, std::stop_token a2,
         const std::chrono::time_point<T2, T3> &a3, T4 a4) {
  return a0.template wait_until<T1, T2, T3, T4>(a1, std::move(a2), a3,
                                                std::move(a4));
}

template <typename T1, typename T2, typename T3, typename T4>
bool f24(std::condition_variable_any &a0, T1 &a1, std::stop_token a2,
         const std::chrono::duration<T2, T3> &a3, T4 a4) {
  return a0.template wait_for<T1, T2, T3, T4>(a1, std::move(a2), a3,
                                              std::move(a4));
}

using t4 = std::cv_status;

std::cv_status f25() { return std::cv_status::no_timeout; }

std::cv_status f26() { return std::cv_status::timeout; }

void f27(std::condition_variable &a0, std::unique_lock<std::mutex> a1) {
  return std::notify_all_at_thread_exit(a0, std::move(a1));
}
