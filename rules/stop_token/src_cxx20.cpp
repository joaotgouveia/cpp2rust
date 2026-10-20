#include "../common/init.h"
#include "../common/traits.h"
#include <stop_token>
#include <utility>

using t1 = std::nostopstate_t;

std::nostopstate_t f1() { return std::nostopstate_t(); }

std::nostopstate_t f2(const std::nostopstate_t &a0) {
  return std::nostopstate_t(a0);
}

std::nostopstate_t f3(std::nostopstate_t &&a0) {
  return std::nostopstate_t(std::move(a0));
}

std::nostopstate_t &f4(std::nostopstate_t &a0, const std::nostopstate_t &a1) {
  return a0.operator=(a1);
}

std::nostopstate_t &f5(std::nostopstate_t &a0, std::nostopstate_t &&a1) {
  return a0.operator=(std::move(a1));
}

using t2 = std::stop_source;

std::stop_source f6() { return std::stop_source(); }

std::stop_source f7(std::nostopstate_t a0) {
  return std::stop_source(std::move(a0));
}

std::stop_source f8(const std::stop_source &a0) { return std::stop_source(a0); }

std::stop_source f9(std::stop_source &&a0) {
  return std::stop_source(std::move(a0));
}

std::stop_source &f10(std::stop_source &a0, const std::stop_source &a1) {
  return a0.operator=(a1);
}

std::stop_source &f11(std::stop_source &a0, std::stop_source &&a1) {
  return a0.operator=(std::move(a1));
}

bool f12(const std::stop_source &a0) { return a0.stop_possible(); }

bool f13(const std::stop_source &a0) { return a0.stop_requested(); }

bool f14(const std::stop_source &a0) { return a0.request_stop(); }

std::stop_token f15(const std::stop_source &a0) { return a0.get_token(); }

void f16(std::stop_source &a0, std::stop_source &a1) { return a0.swap(a1); }

void f17(std::stop_source &a0, std::stop_source &a1) { return swap(a0, a1); }

bool f18(const std::stop_source &a0, const std::stop_source &a1) {
  return a0 == a1;
}

using t3 = std::stop_token;

std::stop_token f19() { return std::stop_token(); }

std::stop_token f20(const std::stop_token &a0) { return std::stop_token(a0); }

std::stop_token f21(std::stop_token &&a0) {
  return std::stop_token(std::move(a0));
}

std::stop_token &f22(std::stop_token &a0, const std::stop_token &a1) {
  return a0.operator=(a1);
}

std::stop_token &f23(std::stop_token &a0, std::stop_token &&a1) {
  return a0.operator=(std::move(a1));
}

bool f24(const std::stop_token &a0) { return a0.stop_possible(); }

bool f25(const std::stop_token &a0) { return a0.stop_requested(); }

void f26(std::stop_token &a0, std::stop_token &a1) { return a0.swap(a1); }

void f27(std::stop_token &a0, std::stop_token &a1) { return swap(a0, a1); }

bool f28(const std::stop_token &a0, const std::stop_token &a1) {
  return a0 == a1;
}
