#include "../common/init.h"
#include "../common/traits.h"
#include <stop_token>
#include <utility>

using t2 = std::nostopstate_t;

std::nostopstate_t f11() { return std::nostopstate_t(); }

std::nostopstate_t f12(const std::nostopstate_t &a0) {
  return std::nostopstate_t(a0);
}

std::nostopstate_t f13(std::nostopstate_t &&a0) {
  return std::nostopstate_t(std::move(a0));
}

std::nostopstate_t &f14(std::nostopstate_t &a0, const std::nostopstate_t &a1) {
  return a0.operator=(a1);
}

std::nostopstate_t &f15(std::nostopstate_t &a0, std::nostopstate_t &&a1) {
  return a0.operator=(std::move(a1));
}

using t3 = std::stop_source;

std::stop_source f16() { return std::stop_source(); }

std::stop_source f17(std::nostopstate_t a0) {
  return std::stop_source(std::move(a0));
}

std::stop_source f18(const std::stop_source &a0) {
  return std::stop_source(a0);
}

std::stop_source f19(std::stop_source &&a0) {
  return std::stop_source(std::move(a0));
}

std::stop_source &f20(std::stop_source &a0, const std::stop_source &a1) {
  return a0.operator=(a1);
}

std::stop_source &f21(std::stop_source &a0, std::stop_source &&a1) {
  return a0.operator=(std::move(a1));
}

bool f22(const std::stop_source &a0) { return a0.stop_possible(); }

bool f23(const std::stop_source &a0) { return a0.stop_requested(); }

bool f24(const std::stop_source &a0) { return a0.request_stop(); }

std::stop_token f25(const std::stop_source &a0) { return a0.get_token(); }

void f26(std::stop_source &a0, std::stop_source &a1) { return a0.swap(a1); }

void f27(std::stop_source &a0, std::stop_source &a1) { return swap(a0, a1); }

bool f28(const std::stop_source &a0, const std::stop_source &a1) {
  return a0 == a1;
}
