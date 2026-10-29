#include <stop_token>
#include <utility>

const std::nostopstate_t &f1() { return std::nostopstate; }

using t1 = std::nostopstate_t;

std::nostopstate_t f2() { return std::nostopstate_t(); }

std::nostopstate_t f3(const std::nostopstate_t &a0) {
  return std::nostopstate_t(a0);
}

std::nostopstate_t f4(std::nostopstate_t &&a0) {
  return std::nostopstate_t(std::move(a0));
}

std::nostopstate_t &f5(std::nostopstate_t &a0, const std::nostopstate_t &a1) {
  return a0.operator=(a1);
}

std::nostopstate_t &f6(std::nostopstate_t &a0, std::nostopstate_t &&a1) {
  return a0.operator=(std::move(a1));
}

using t2 = std::stop_source;

std::stop_source f7() { return std::stop_source(); }

std::stop_source f8(std::nostopstate_t a0) {
  return std::stop_source(std::move(a0));
}

std::stop_source f9(const std::stop_source &a0) { return std::stop_source(a0); }

std::stop_source f10(std::stop_source &&a0) {
  return std::stop_source(std::move(a0));
}

std::stop_source &f11(std::stop_source &a0, const std::stop_source &a1) {
  return a0.operator=(a1);
}

std::stop_source &f12(std::stop_source &a0, std::stop_source &&a1) {
  return a0.operator=(std::move(a1));
}

bool f13(const std::stop_source &a0) { return a0.stop_possible(); }

bool f14(const std::stop_source &a0) { return a0.stop_requested(); }

bool f15(const std::stop_source &a0) { return a0.request_stop(); }

std::stop_token f16(const std::stop_source &a0) { return a0.get_token(); }

void f17(std::stop_source &a0, std::stop_source &a1) { return a0.swap(a1); }

void f18(std::stop_source &a0, std::stop_source &a1) { return swap(a0, a1); }

bool f19(const std::stop_source &a0, const std::stop_source &a1) {
  return operator==(a0, a1);
}

using t3 = std::stop_token;

std::stop_token f20() { return std::stop_token(); }

std::stop_token f21(const std::stop_token &a0) { return std::stop_token(a0); }

std::stop_token f22(std::stop_token &&a0) {
  return std::stop_token(std::move(a0));
}

std::stop_token &f23(std::stop_token &a0, const std::stop_token &a1) {
  return a0.operator=(a1);
}

std::stop_token &f24(std::stop_token &a0, std::stop_token &&a1) {
  return a0.operator=(std::move(a1));
}

bool f25(const std::stop_token &a0) { return a0.stop_possible(); }

bool f26(const std::stop_token &a0) { return a0.stop_requested(); }

void f27(std::stop_token &a0, std::stop_token &a1) { return a0.swap(a1); }

void f28(std::stop_token &a0, std::stop_token &a1) { return swap(a0, a1); }

bool f29(const std::stop_token &a0, const std::stop_token &a1) {
  return operator==(a0, a1);
}
