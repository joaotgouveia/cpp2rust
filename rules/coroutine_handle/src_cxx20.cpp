#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <coroutine>
#include <utility>

using t1 = std::coroutine_handle<std::noop_coroutine_promise>;

std::coroutine_handle<>
f1(const std::coroutine_handle<std::noop_coroutine_promise> &a0) {
  return static_cast<std::coroutine_handle<>>(a0);
}

bool f2(const std::coroutine_handle<std::noop_coroutine_promise> &a0) {
  return static_cast<bool>(a0);
}

bool f3(const std::coroutine_handle<std::noop_coroutine_promise> &a0) {
  return a0.done();
}

void f4(const std::coroutine_handle<std::noop_coroutine_promise> &a0) {
  return a0.operator()();
}

void f5(const std::coroutine_handle<std::noop_coroutine_promise> &a0) {
  return a0.resume();
}

void f6(const std::coroutine_handle<std::noop_coroutine_promise> &a0) {
  return a0.destroy();
}

std::noop_coroutine_promise &
f7(const std::coroutine_handle<std::noop_coroutine_promise> &a0) {
  return a0.promise();
}

void *f8(const std::coroutine_handle<std::noop_coroutine_promise> &a0) {
  return a0.address();
}

std::coroutine_handle<std::noop_coroutine_promise>
f9(const std::coroutine_handle<std::noop_coroutine_promise> &a0) {
  return std::coroutine_handle<std::noop_coroutine_promise>(a0);
}

std::coroutine_handle<std::noop_coroutine_promise>
f10(std::coroutine_handle<std::noop_coroutine_promise> &&a0) {
  return std::coroutine_handle<std::noop_coroutine_promise>(std::move(a0));
}

std::coroutine_handle<std::noop_coroutine_promise> &
f11(std::coroutine_handle<std::noop_coroutine_promise> &a0,
    const std::coroutine_handle<std::noop_coroutine_promise> &a1) {
  return a0.operator=(a1);
}

std::coroutine_handle<std::noop_coroutine_promise> &
f12(std::coroutine_handle<std::noop_coroutine_promise> &a0,
    std::coroutine_handle<std::noop_coroutine_promise> &&a1) {
  return a0.operator=(std::move(a1));
}

using t2 = std::coroutine_handle<void>;

std::coroutine_handle<void> f13() { return std::coroutine_handle<void>(); }

std::coroutine_handle<void> f14(std::nullptr_t a0) {
  return std::coroutine_handle<void>(a0);
}

std::coroutine_handle<void> &f15(std::coroutine_handle<void> &a0,
                                 std::nullptr_t a1) {
  return a0.operator=(a1);
}

void *f16(const std::coroutine_handle<void> &a0) { return a0.address(); }

std::coroutine_handle<void> f17(void *a0) {
  return std::coroutine_handle<>::from_address(a0);
}

bool f18(const std::coroutine_handle<void> &a0) {
  return static_cast<bool>(a0);
}

bool f19(const std::coroutine_handle<void> &a0) { return a0.done(); }

void f20(const std::coroutine_handle<void> &a0) { return a0.operator()(); }

void f21(const std::coroutine_handle<void> &a0) { return a0.resume(); }

void f22(const std::coroutine_handle<void> &a0) { return a0.destroy(); }

std::coroutine_handle<void> f23(const std::coroutine_handle<void> &a0) {
  return std::coroutine_handle<void>(a0);
}

std::coroutine_handle<void> f24(std::coroutine_handle<void> &&a0) {
  return std::coroutine_handle<void>(std::move(a0));
}

std::coroutine_handle<void> &f25(std::coroutine_handle<void> &a0,
                                 const std::coroutine_handle<void> &a1) {
  return a0.operator=(a1);
}

std::coroutine_handle<void> &f26(std::coroutine_handle<void> &a0,
                                 std::coroutine_handle<void> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> using t3 = std::coroutine_handle<T1>;

template <typename T1> std::coroutine_handle<T1> f27() {
  return std::coroutine_handle<T1>();
}

template <typename T1> std::coroutine_handle<T1> f28(std::nullptr_t a0) {
  return std::coroutine_handle<T1>(a0);
}

template <typename T1> std::coroutine_handle<T1> f29(T1 &a0) {
  return std::coroutine_handle<T1>::from_promise(a0);
}

template <typename T1>
std::coroutine_handle<T1> &f30(std::coroutine_handle<T1> &a0,
                               std::nullptr_t a1) {
  return a0.operator=(a1);
}

template <typename T1> void *f31(const std::coroutine_handle<T1> &a0) {
  return a0.address();
}

template <typename T1> std::coroutine_handle<T1> f32(void *a0) {
  return std::coroutine_handle<T1>::from_address(a0);
}

template <typename T1>
std::coroutine_handle<> f33(const std::coroutine_handle<T1> &a0) {
  return static_cast<std::coroutine_handle<>>(a0);
}

template <typename T1> bool f34(const std::coroutine_handle<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1> bool f35(const std::coroutine_handle<T1> &a0) {
  return a0.done();
}

template <typename T1> void f36(const std::coroutine_handle<T1> &a0) {
  return a0.operator()();
}

template <typename T1> void f37(const std::coroutine_handle<T1> &a0) {
  return a0.resume();
}

template <typename T1> void f38(const std::coroutine_handle<T1> &a0) {
  return a0.destroy();
}

template <typename T1> T1 &f39(const std::coroutine_handle<T1> &a0) {
  return a0.promise();
}
