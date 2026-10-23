#include <coroutine>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

using t1 = std::suspend_always;

bool f1(const std::suspend_always &a0) { return a0.await_ready(); }

void f2(const std::suspend_always &a0, std::coroutine_handle<> a1) {
  return a0.await_suspend(std::move(a1));
}

void f3(const std::suspend_always &a0) { return a0.await_resume(); }

std::suspend_always f4() { return std::suspend_always(); }

std::suspend_always f5(const std::suspend_always &a0) {
  return std::suspend_always(a0);
}

std::suspend_always f6(std::suspend_always &&a0) {
  return std::suspend_always(std::move(a0));
}

std::suspend_always &f7(std::suspend_always &a0,
                        const std::suspend_always &a1) {
  return a0.operator=(a1);
}

std::suspend_always &f8(std::suspend_always &a0, std::suspend_always &&a1) {
  return a0.operator=(std::move(a1));
}

using t2 = std::suspend_never;

bool f9(const std::suspend_never &a0) { return a0.await_ready(); }

void f10(const std::suspend_never &a0, std::coroutine_handle<> a1) {
  return a0.await_suspend(std::move(a1));
}

void f11(const std::suspend_never &a0) { return a0.await_resume(); }

std::suspend_never f12() { return std::suspend_never(); }

std::suspend_never f13(const std::suspend_never &a0) {
  return std::suspend_never(a0);
}

std::suspend_never f14(std::suspend_never &&a0) {
  return std::suspend_never(std::move(a0));
}

std::suspend_never &f15(std::suspend_never &a0, const std::suspend_never &a1) {
  return a0.operator=(a1);
}

std::suspend_never &f16(std::suspend_never &a0, std::suspend_never &&a1) {
  return a0.operator=(std::move(a1));
}
