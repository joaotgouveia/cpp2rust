#include "../common/init.h"
#include "../common/traits.h"
#include <future>
#include <system_error>
#include <utility>

template <typename T1, typename... T2>
auto f1(std::launch a0, T1 &&a1, T2 &&...a2)
    -> decltype(std::async<T1, T2...>(a0, std::forward<T1>(a1),
                                      std::forward<T2>(a2)...)) {
  return std::async<T1, T2...>(a0, std::forward<T1>(a1),
                               std::forward<T2>(a2)...);
}

template <typename T1, typename... T2>
auto f2(T1 &&a0, T2 &&...a1)
    -> decltype(std::async<T1, T2...>(std::forward<T1>(a0),
                                      std::forward<T2>(a1)...)) {
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

const std::error_category &f10() { return std::future_category(); }

using t2 = std::future_errc;

std::future_errc f11() { return std::future_errc::future_already_retrieved; }

std::future_errc f12() { return std::future_errc::promise_already_satisfied; }

std::future_errc f13() { return std::future_errc::no_state; }

std::future_errc f14() { return std::future_errc::broken_promise; }

using t3 = std::future_error;

std::future_error f15(std::future_errc a0) { return std::future_error(a0); }

const char *f16(const std::future_error &a0) { return a0.what(); }

const std::error_code &f17(const std::future_error &a0) { return a0.code(); }

std::future_error f18(const std::future_error &a0) {
  return std::future_error(a0);
}

std::future_error &f19(std::future_error &a0, const std::future_error &a1) {
  return a0.operator=(a1);
}

using t4 = std::future_status;

std::future_status f20() { return std::future_status::ready; }

std::future_status f21() { return std::future_status::timeout; }

std::future_status f22() { return std::future_status::deferred; }

using t5 = std::launch;

std::launch f23() { return std::launch::async; }

std::launch f24() { return std::launch::deferred; }

std::error_code f25(std::future_errc a0) { return std::make_error_code(a0); }

std::error_condition f26(std::future_errc a0) {
  return std::make_error_condition(a0);
}

std::launch f27(std::launch a0, std::launch a1) { return a0 & a1; }

std::launch &f28(std::launch &a0, std::launch a1) {
  return std::operator&=(a0, a1);
}

std::launch f29(std::launch a0, std::launch a1) { return a0 ^ a1; }

std::launch &f30(std::launch &a0, std::launch a1) {
  return std::operator^=(a0, a1);
}

std::launch f31(std::launch a0, std::launch a1) { return a0 | a1; }

std::launch &f32(std::launch &a0, std::launch a1) {
  return std::operator|=(a0, a1);
}

std::launch f33(std::launch a0) { return ~a0; }

template <typename T1> void f34(std::promise<T1> &a0, std::promise<T1> &a1) {
  return std::swap<T1>(a0, a1);
}

template <typename T1, typename... T2>
void f35(std::packaged_task<T1(T2...)> &a0, std::packaged_task<T1(T2...)> &a1) {
  return std::swap<T1, T2...>(a0, a1);
}
