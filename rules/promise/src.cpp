#include "../common/init.h"
#include "../common/traits.h"
#include <future>
#include <tuple>
#include <utility>

using t1 = std::promise<void>;

std::promise<void> f1() { return std::promise<void>(); }

std::promise<void> f2(std::promise<void> &&a0) {
  return std::promise<void>(std::move(a0));
}

std::promise<void> &f3(std::promise<void> &a0, std::promise<void> &&a1) {
  return a0.operator=(std::move(a1));
}

void f4(std::promise<void> &a0, std::promise<void> &a1) { return a0.swap(a1); }

std::future<void> f5(std::promise<void> &a0) { return a0.get_future(); }

void f6(std::promise<void> &a0) { return a0.set_value(); }

void f7(std::promise<void> &a0, std::exception_ptr a1) {
  return a0.set_exception(std::move(a1));
}

void f8(std::promise<void> &a0) { return a0.set_value_at_thread_exit(); }

void f9(std::promise<void> &a0, std::exception_ptr a1) {
  return a0.set_exception_at_thread_exit(std::move(a1));
}

template <typename T1>
std::promise<void> f10(std::allocator_arg_t a0, const T1 &a1) {
  return std::promise<void>(std::move(a0), a1);
}

template <typename T1>
std::promise<void> f11(std::allocator_arg_t a0, const T1 &a1,
                       std::promise<void> &&a2) {
  return std::promise<void>(std::move(a0), a1, std::move(a2));
}

template <typename T1> using t2 = std::promise<T1 &>;

template <typename T1> std::promise<T1 &> f12() { return std::promise<T1 &>(); }

template <typename T1> std::promise<T1 &> f13(std::promise<T1 &> &&a0) {
  return std::promise<T1 &>(std::move(a0));
}

template <typename T1>
std::promise<T1 &> &f14(std::promise<T1 &> &a0, std::promise<T1 &> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
void f15(std::promise<T1 &> &a0, std::promise<T1 &> &a1) {
  return a0.swap(a1);
}

template <typename T1> std::future<T1 &> f16(std::promise<T1 &> &a0) {
  return a0.get_future();
}

template <typename T1> void f17(std::promise<T1 &> &a0, T1 &a1) {
  return a0.set_value(a1);
}

template <typename T1> void f18(std::promise<T1 &> &a0, std::exception_ptr a1) {
  return a0.set_exception(std::move(a1));
}

template <typename T1> void f19(std::promise<T1 &> &a0, T1 &a1) {
  return a0.set_value_at_thread_exit(a1);
}

template <typename T1> void f20(std::promise<T1 &> &a0, std::exception_ptr a1) {
  return a0.set_exception_at_thread_exit(std::move(a1));
}

template <typename T1, typename T2>
std::promise<T1 &> f21(std::allocator_arg_t a0, const T2 &a1) {
  return std::promise<T1 &>(std::move(a0), a1);
}

template <typename T1, typename T2>
std::promise<T1 &> f22(std::allocator_arg_t a0, const T2 &a1,
                       std::promise<T1 &> &&a2) {
  return std::promise<T1 &>(std::move(a0), a1, std::move(a2));
}

template <typename T1> using t3 = std::promise<T1>;

template <typename T1> std::promise<T1> f23() { return std::promise<T1>(); }

template <typename T1> std::promise<T1> f24(std::promise<T1> &&a0) {
  return std::promise<T1>(std::move(a0));
}

template <typename T1>
std::promise<T1> &f25(std::promise<T1> &a0, std::promise<T1> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> void f26(std::promise<T1> &a0, std::promise<T1> &a1) {
  return a0.swap(a1);
}

template <typename T1> std::future<T1> f27(std::promise<T1> &a0) {
  return a0.get_future();
}

template <typename T1> void f28(std::promise<T1> &a0, const T1 &a1) {
  return a0.set_value(a1);
}

template <typename T1> void f29(std::promise<T1> &a0, T1 &&a1) {
  return a0.set_value(std::move(a1));
}

template <typename T1> void f30(std::promise<T1> &a0, std::exception_ptr a1) {
  return a0.set_exception(std::move(a1));
}

template <typename T1> void f31(std::promise<T1> &a0, const T1 &a1) {
  return a0.set_value_at_thread_exit(a1);
}

template <typename T1> void f32(std::promise<T1> &a0, T1 &&a1) {
  return a0.set_value_at_thread_exit(std::move(a1));
}

template <typename T1> void f33(std::promise<T1> &a0, std::exception_ptr a1) {
  return a0.set_exception_at_thread_exit(std::move(a1));
}

template <typename T1, typename T2>
std::promise<T1> f34(std::allocator_arg_t a0, const T2 &a1) {
  return std::promise<T1>(std::move(a0), a1);
}

template <typename T1, typename T2>
std::promise<T1> f35(std::allocator_arg_t a0, const T2 &a1,
                     std::promise<T1> &&a2) {
  return std::promise<T1>(std::move(a0), a1, std::move(a2));
}
