#include <barrier>
#include <compare>
#include <stop_token>
#include <thread>
#include <utility>

using t1 = std::jthread;

using t2 = std::jthread::id;

using t3 = std::jthread::native_handle_type;

std::jthread f7() { return std::jthread(); }

std::jthread f8(std::jthread &&a0) { return std::jthread(std::move(a0)); }

std::jthread &f9(std::jthread &a0, std::jthread &&a1) {
  return a0.operator=(std::move(a1));
}

void f10(std::jthread &a0, std::jthread &a1) { return a0.swap(a1); }

bool f11(const std::jthread &a0) { return a0.joinable(); }

void f12(std::jthread &a0) { return a0.join(); }

void f13(std::jthread &a0) { return a0.detach(); }

std::jthread::id f14(const std::jthread &a0) { return a0.get_id(); }

std::jthread::native_handle_type f15(std::jthread &a0) {
  return a0.native_handle();
}

unsigned int f16() { return std::jthread::hardware_concurrency(); }

std::stop_source f17(std::jthread &a0) { return a0.get_stop_source(); }

std::stop_token f18(const std::jthread &a0) { return a0.get_stop_token(); }

bool f19(std::jthread &a0) { return a0.request_stop(); }

template <typename T1, typename... T2,
          typename T3 = std::enable_if_t<
              !std::is_same_v<std::remove_cvref_t<T1>, std::jthread>>>
std::jthread f20(T1 &&a0, T2 &&...a1) {
  return std::jthread(std::forward<T1>(a0), std::forward<T2>(a1)...);
}

void f21(std::jthread &a0, std::jthread &a1) { return swap(a0, a1); }

std::strong_ordering f22(std::thread::id a0, std::thread::id a1) {
  return std::move(a0) <=> std::move(a1);
}
