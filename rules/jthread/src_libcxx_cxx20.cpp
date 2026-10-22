#include "../common/init.h"
#include "../common/traits.h"
#include <stop_token>
#include <thread>
#include <utility>

using t1 = std::jthread;

using t2 = std::jthread::id;

using t3 = std::jthread::native_handle_type;

std::jthread f1() { return std::jthread(); }

std::jthread f2(std::jthread &&a0) { return std::jthread(std::move(a0)); }

std::jthread &f3(std::jthread &a0, std::jthread &&a1) {
  return a0.operator=(std::move(a1));
}

void f4(std::jthread &a0, std::jthread &a1) { return a0.swap(a1); }

bool f5(const std::jthread &a0) { return a0.joinable(); }

void f6(std::jthread &a0) { return a0.join(); }

void f7(std::jthread &a0) { return a0.detach(); }

std::jthread::id f8(const std::jthread &a0) { return a0.get_id(); }

std::jthread::native_handle_type f9(std::jthread &a0) {
  return a0.native_handle();
}

std::stop_source f10(std::jthread &a0) { return a0.get_stop_source(); }

std::stop_token f11(const std::jthread &a0) { return a0.get_stop_token(); }

bool f12(std::jthread &a0) { return a0.request_stop(); }

unsigned int f13() { return std::jthread::hardware_concurrency(); }

template <typename T1, typename... T2> std::jthread f14(T1 &&a0, T2 &&...a1) {
  return std::jthread(std::forward<T1>(a0), std::forward<T2>(a1)...);
}

void f15(std::jthread &a0, std::jthread &a1) { return swap(a0, a1); }

std::jthread::id f16() { return std::jthread::id(); }

std::jthread::id f17(const std::jthread::id &a0) {
  return std::jthread::id(a0);
}

std::jthread::id f18(std::jthread::id &&a0) {
  return std::jthread::id(std::move(a0));
}

std::jthread::id &f19(std::jthread::id &a0, const std::jthread::id &a1) {
  return a0.operator=(a1);
}

std::jthread::id &f20(std::jthread::id &a0, std::jthread::id &&a1) {
  return a0.operator=(std::move(a1));
}
