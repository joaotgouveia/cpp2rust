#include <compare>
#include <coroutine>
#include <utility>

std::strong_ordering f1(std::coroutine_handle<> a0,
                        std::coroutine_handle<> a1) {
  return std::move(a0) <=> std::move(a1);
}

bool f2(std::coroutine_handle<> a0, std::coroutine_handle<> a1) {
  return std::move(a0) == std::move(a1);
}

std::noop_coroutine_handle f3() { return std::noop_coroutine(); }

using t1 = std::noop_coroutine_handle;

using t2 = std::noop_coroutine_promise;

std::noop_coroutine_promise f4() { return std::noop_coroutine_promise(); }

std::noop_coroutine_promise f5(const std::noop_coroutine_promise &a0) {
  return std::noop_coroutine_promise(a0);
}

std::noop_coroutine_promise f6(std::noop_coroutine_promise &&a0) {
  return std::noop_coroutine_promise(std::move(a0));
}

std::noop_coroutine_promise &f7(std::noop_coroutine_promise &a0,
                                const std::noop_coroutine_promise &a1) {
  return a0.operator=(a1);
}

std::noop_coroutine_promise &f8(std::noop_coroutine_promise &a0,
                                std::noop_coroutine_promise &&a1) {
  return a0.operator=(std::move(a1));
}

using t3 = std::suspend_always;

bool f9(const std::suspend_always &a0) { return a0.await_ready(); }

void f10(const std::suspend_always &a0, std::coroutine_handle<> a1) {
  return a0.await_suspend(std::move(a1));
}

void f11(const std::suspend_always &a0) { return a0.await_resume(); }

std::suspend_always f12() { return std::suspend_always(); }

std::suspend_always f13(const std::suspend_always &a0) {
  return std::suspend_always(a0);
}

std::suspend_always f14(std::suspend_always &&a0) {
  return std::suspend_always(std::move(a0));
}

std::suspend_always &f15(std::suspend_always &a0,
                         const std::suspend_always &a1) {
  return a0.operator=(a1);
}

std::suspend_always &f16(std::suspend_always &a0, std::suspend_always &&a1) {
  return a0.operator=(std::move(a1));
}

using t4 = std::suspend_never;

bool f17(const std::suspend_never &a0) { return a0.await_ready(); }

void f18(const std::suspend_never &a0, std::coroutine_handle<> a1) {
  return a0.await_suspend(std::move(a1));
}

void f19(const std::suspend_never &a0) { return a0.await_resume(); }

std::suspend_never f20() { return std::suspend_never(); }

std::suspend_never f21(const std::suspend_never &a0) {
  return std::suspend_never(a0);
}

std::suspend_never f22(std::suspend_never &&a0) {
  return std::suspend_never(std::move(a0));
}

std::suspend_never &f23(std::suspend_never &a0, const std::suspend_never &a1) {
  return a0.operator=(a1);
}

std::suspend_never &f24(std::suspend_never &a0, std::suspend_never &&a1) {
  return a0.operator=(std::move(a1));
}
