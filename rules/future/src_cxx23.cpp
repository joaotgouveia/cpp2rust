#include <chrono>
#include <future>
#include <system_error>
#include <utility>

template <typename T1, typename... T2>
auto f1(std::launch a0, T1 &&a1, T2 &&...a2) {
  return std::async<T1, T2...>(a0, std::forward<T1>(a1),
                               std::forward<T2>(a2)...);
}

template <typename T1, typename... T2> auto f2(T1 &&a0, T2 &&...a1) {
  return std::async<T1, T2...>(std::forward<T1>(a0), std::forward<T2>(a1)...);
}

template <typename T1> using t1 = std::future<T1>;

template <typename T1> std::future<T1> f3() { return std::future<T1>(); }

template <typename T1> std::future<T1> f4(std::future<T1> &&a0) {
  return std::future<T1>(std::move(a0));
}

template <typename T1>
std::future<T1> &f5(std::future<T1> &a0, std::future<T1> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> T1 f6(std::future<T1> &a0) { return a0.get(); }

template <typename T1> std::shared_future<T1> f7(std::future<T1> &a0) {
  return a0.share();
}

template <typename T1> bool f8(const std::future<T1> &a0) { return a0.valid(); }

template <typename T1> void f9(const std::future<T1> &a0) { return a0.wait(); }

template <typename T1, typename T2, typename T3>
std::future_status f10(const std::future<T1> &a0,
                       const std::chrono::duration<T2, T3> &a1) {
  return a0.template wait_for<T2, T3>(a1);
}

template <typename T1, typename T2, typename T3>
std::future_status f11(const std::future<T1> &a0,
                       const std::chrono::time_point<T2, T3> &a1) {
  return a0.template wait_until<T2, T3>(a1);
}

const std::error_category &f12() { return std::future_category(); }

using t2 = std::future_errc;

std::future_errc f13() { return std::future_errc::future_already_retrieved; }

std::future_errc f14() { return std::future_errc::promise_already_satisfied; }

std::future_errc f15() { return std::future_errc::no_state; }

std::future_errc f16() { return std::future_errc::broken_promise; }

using t3 = std::future_error;

std::future_error f17(std::future_errc a0) { return std::future_error(a0); }

const char *f18(const std::future_error &a0) { return a0.what(); }

const std::error_code &f19(const std::future_error &a0) { return a0.code(); }

std::future_error f20(const std::future_error &a0) {
  return std::future_error(a0);
}

std::future_error &f21(std::future_error &a0, const std::future_error &a1) {
  return a0.operator=(a1);
}

using t4 = std::future_status;

std::future_status f22() { return std::future_status::ready; }

std::future_status f23() { return std::future_status::timeout; }

std::future_status f24() { return std::future_status::deferred; }

using t5 = std::launch;

std::launch f25() { return std::launch::async; }

std::launch f26() { return std::launch::deferred; }

std::error_code f27(std::future_errc a0) { return std::make_error_code(a0); }

std::error_condition f28(std::future_errc a0) {
  return std::make_error_condition(a0);
}

std::launch f29(std::launch a0, std::launch a1) {
  return std::operator&(a0, a1);
}

std::launch &f30(std::launch &a0, std::launch a1) {
  return std::operator&=(a0, a1);
}

std::launch f31(std::launch a0, std::launch a1) {
  return std::operator^(a0, a1);
}

std::launch &f32(std::launch &a0, std::launch a1) {
  return std::operator^=(a0, a1);
}

std::launch f33(std::launch a0, std::launch a1) {
  return std::operator|(a0, a1);
}

std::launch &f34(std::launch &a0, std::launch a1) {
  return std::operator|=(a0, a1);
}

std::launch f35(std::launch a0) { return std::operator~(a0); }

template <typename T1> void f36(std::promise<T1> &a0, std::promise<T1> &a1) {
  return std::swap<T1>(a0, a1);
}

template <typename T1, typename... T2>
void f37(std::packaged_task<T1(T2...)> &a0, std::packaged_task<T1(T2...)> &a1) {
  return std::swap<T1, T2...>(a0, a1);
}
