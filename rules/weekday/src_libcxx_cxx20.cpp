#include "../common/init.h"
#include "../common/traits.h"
#include <chrono>
#include <utility>

std::chrono::weekday f1(const std::chrono::days &a0,
                        const std::chrono::weekday &a1) {
  return a0 + a1;
}

std::chrono::weekday f2(const std::chrono::weekday &a0,
                        const std::chrono::days &a1) {
  return a0 + a1;
}

std::chrono::weekday f3(const std::chrono::weekday &a0,
                        const std::chrono::days &a1) {
  return a0 - a1;
}

std::chrono::days f4(const std::chrono::weekday &a0,
                     const std::chrono::weekday &a1) {
  return a0 - a1;
}

bool f5(const std::chrono::weekday &a0, const std::chrono::weekday &a1) {
  return a0 == a1;
}

bool f6(const std::chrono::weekday_indexed &a0,
        const std::chrono::weekday_indexed &a1) {
  return a0 == a1;
}

bool f7(const std::chrono::weekday_last &a0,
        const std::chrono::weekday_last &a1) {
  return a0 == a1;
}

using t1 = std::chrono::weekday;

std::chrono::weekday f8() { return std::chrono::weekday(); }

std::chrono::weekday f9(unsigned int a0) { return std::chrono::weekday(a0); }

std::chrono::weekday f10(const std::chrono::sys_days &a0) {
  return std::chrono::weekday(a0);
}

std::chrono::weekday f11(const std::chrono::local_days &a0) {
  return std::chrono::weekday(a0);
}

std::chrono::weekday &f12(std::chrono::weekday &a0) { return a0.operator++(); }

std::chrono::weekday f13(std::chrono::weekday &a0, int a1) {
  return a0.operator++(a1);
}

std::chrono::weekday &f14(std::chrono::weekday &a0) { return a0.operator--(); }

std::chrono::weekday f15(std::chrono::weekday &a0, int a1) {
  return a0.operator--(a1);
}

std::chrono::weekday &f16(std::chrono::weekday &a0,
                          const std::chrono::days &a1) {
  return a0.operator+=(a1);
}

std::chrono::weekday &f17(std::chrono::weekday &a0,
                          const std::chrono::days &a1) {
  return a0.operator-=(a1);
}

unsigned int f18(const std::chrono::weekday &a0) { return a0.c_encoding(); }

unsigned int f19(const std::chrono::weekday &a0) { return a0.iso_encoding(); }

bool f20(const std::chrono::weekday &a0) { return a0.ok(); }

std::chrono::weekday_indexed f21(const std::chrono::weekday &a0,
                                 unsigned int a1) {
  return a0.operator[](a1);
}

std::chrono::weekday_last f22(const std::chrono::weekday &a0,
                              std::chrono::last_spec a1) {
  return a0.operator[](std::move(a1));
}

std::chrono::weekday f23(const std::chrono::weekday &a0) {
  return std::chrono::weekday(a0);
}

std::chrono::weekday f24(std::chrono::weekday &&a0) {
  return std::chrono::weekday(std::move(a0));
}

std::chrono::weekday &f25(std::chrono::weekday &a0,
                          const std::chrono::weekday &a1) {
  return a0.operator=(a1);
}

std::chrono::weekday &f26(std::chrono::weekday &a0, std::chrono::weekday &&a1) {
  return a0.operator=(std::move(a1));
}

using t2 = std::chrono::weekday_indexed;

std::chrono::weekday_indexed f27() { return std::chrono::weekday_indexed(); }

std::chrono::weekday_indexed f28(const std::chrono::weekday &a0,
                                 unsigned int a1) {
  return std::chrono::weekday_indexed(a0, a1);
}

std::chrono::weekday f29(const std::chrono::weekday_indexed &a0) {
  return a0.weekday();
}

unsigned int f30(const std::chrono::weekday_indexed &a0) { return a0.index(); }

bool f31(const std::chrono::weekday_indexed &a0) { return a0.ok(); }

std::chrono::weekday_indexed f32(const std::chrono::weekday_indexed &a0) {
  return std::chrono::weekday_indexed(a0);
}

std::chrono::weekday_indexed f33(std::chrono::weekday_indexed &&a0) {
  return std::chrono::weekday_indexed(std::move(a0));
}

std::chrono::weekday_indexed &f34(std::chrono::weekday_indexed &a0,
                                  const std::chrono::weekday_indexed &a1) {
  return a0.operator=(a1);
}

std::chrono::weekday_indexed &f35(std::chrono::weekday_indexed &a0,
                                  std::chrono::weekday_indexed &&a1) {
  return a0.operator=(std::move(a1));
}

using t3 = std::chrono::weekday_last;

std::chrono::weekday_last f36(const std::chrono::weekday &a0) {
  return std::chrono::weekday_last(a0);
}

std::chrono::weekday f37(const std::chrono::weekday_last &a0) {
  return a0.weekday();
}

bool f38(const std::chrono::weekday_last &a0) { return a0.ok(); }

std::chrono::weekday_last f39(const std::chrono::weekday_last &a0) {
  return std::chrono::weekday_last(a0);
}

std::chrono::weekday_last f40(std::chrono::weekday_last &&a0) {
  return std::chrono::weekday_last(std::move(a0));
}

std::chrono::weekday_last &f41(std::chrono::weekday_last &a0,
                               const std::chrono::weekday_last &a1) {
  return a0.operator=(a1);
}

std::chrono::weekday_last &f42(std::chrono::weekday_last &a0,
                               std::chrono::weekday_last &&a1) {
  return a0.operator=(std::move(a1));
}
