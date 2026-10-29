#include <barrier>
#include <compare>
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

unsigned int f10() { return std::jthread::hardware_concurrency(); }

std::stop_source f11(std::jthread &a0) { return a0.get_stop_source(); }

std::stop_token f12(const std::jthread &a0) { return a0.get_stop_token(); }

bool f13(std::jthread &a0) { return a0.request_stop(); }

template <typename T1, typename... T2,
          typename T3 = std::enable_if_t<
              !std::is_same_v<std::remove_cvref_t<T1>, std::jthread>>>
std::jthread f14(T1 &&a0, T2 &&...a1) {
  return std::jthread(std::forward<T1>(a0), std::forward<T2>(a1)...);
}

void f15(std::jthread &a0, std::jthread &a1) { return swap(a0, a1); }

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f16(std::basic_ostream<T1, T2> &a0,
                                std::thread::id a1) {
  return std::operator<<(a0, std::move(a1));
}

std::strong_ordering f17(std::thread::id a0, std::thread::id a1) {
  return std::operator<=>(std::move(a0), std::move(a1));
}
