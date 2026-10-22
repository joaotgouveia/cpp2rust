#include "../common/init.h"
#include "../common/traits.h"
#include <chrono>
#include <utility>

std::chrono::year_month_weekday f1(const std::chrono::months &a0,
                                   const std::chrono::year_month_weekday &a1) {
  return a0 + a1;
}

std::chrono::year_month_weekday_last
f2(const std::chrono::months &a0,
   const std::chrono::year_month_weekday_last &a1) {
  return a0 + a1;
}

std::chrono::year_month_weekday f3(const std::chrono::years &a0,
                                   const std::chrono::year_month_weekday &a1) {
  return a0 + a1;
}

std::chrono::year_month_weekday_last
f4(const std::chrono::years &a0,
   const std::chrono::year_month_weekday_last &a1) {
  return a0 + a1;
}

std::chrono::year_month_weekday f5(const std::chrono::year_month_weekday &a0,
                                   const std::chrono::months &a1) {
  return a0 + a1;
}

std::chrono::year_month_weekday f6(const std::chrono::year_month_weekday &a0,
                                   const std::chrono::years &a1) {
  return a0 + a1;
}

std::chrono::year_month_weekday_last
f7(const std::chrono::year_month_weekday_last &a0,
   const std::chrono::months &a1) {
  return a0 + a1;
}

std::chrono::year_month_weekday_last
f8(const std::chrono::year_month_weekday_last &a0,
   const std::chrono::years &a1) {
  return a0 + a1;
}

std::chrono::year_month_weekday f9(const std::chrono::year_month_weekday &a0,
                                   const std::chrono::months &a1) {
  return a0 - a1;
}

std::chrono::year_month_weekday f10(const std::chrono::year_month_weekday &a0,
                                    const std::chrono::years &a1) {
  return a0 - a1;
}

std::chrono::year_month_weekday_last
f11(const std::chrono::year_month_weekday_last &a0,
    const std::chrono::months &a1) {
  return a0 - a1;
}

std::chrono::year_month_weekday_last
f12(const std::chrono::year_month_weekday_last &a0,
    const std::chrono::years &a1) {
  return a0 - a1;
}

std::chrono::year_month_weekday f13(const std::chrono::month_weekday &a0,
                                    const std::chrono::year &a1) {
  return a0 / a1;
}

std::chrono::year_month_weekday f14(const std::chrono::month_weekday &a0,
                                    int a1) {
  return a0 / a1;
}

std::chrono::year_month_weekday_last
f15(const std::chrono::month_weekday_last &a0, const std::chrono::year &a1) {
  return a0 / a1;
}

std::chrono::year_month_weekday_last
f16(const std::chrono::month_weekday_last &a0, int a1) {
  return a0 / a1;
}

std::chrono::year_month_weekday f17(const std::chrono::year &a0,
                                    const std::chrono::month_weekday &a1) {
  return a0 / a1;
}

std::chrono::year_month_weekday_last
f18(const std::chrono::year &a0, const std::chrono::month_weekday_last &a1) {
  return a0 / a1;
}

std::chrono::year_month_weekday f19(const std::chrono::year_month &a0,
                                    const std::chrono::weekday_indexed &a1) {
  return a0 / a1;
}

std::chrono::year_month_weekday_last f20(const std::chrono::year_month &a0,
                                         const std::chrono::weekday_last &a1) {
  return a0 / a1;
}

std::chrono::year_month_weekday f21(int a0,
                                    const std::chrono::month_weekday &a1) {
  return a0 / a1;
}

std::chrono::year_month_weekday_last
f22(int a0, const std::chrono::month_weekday_last &a1) {
  return a0 / a1;
}

bool f23(const std::chrono::year_month_weekday &a0,
         const std::chrono::year_month_weekday &a1) {
  return a0 == a1;
}

bool f24(const std::chrono::year_month_weekday_last &a0,
         const std::chrono::year_month_weekday_last &a1) {
  return a0 == a1;
}

using t1 = std::chrono::year_month_weekday;

std::chrono::year_month_weekday f25() {
  return std::chrono::year_month_weekday();
}

std::chrono::year_month_weekday f26(const std::chrono::year &a0,
                                    const std::chrono::month &a1,
                                    const std::chrono::weekday_indexed &a2) {
  return std::chrono::year_month_weekday(a0, a1, a2);
}

std::chrono::year_month_weekday f27(const std::chrono::sys_days &a0) {
  return std::chrono::year_month_weekday(a0);
}

std::chrono::year_month_weekday f28(const std::chrono::local_days &a0) {
  return std::chrono::year_month_weekday(a0);
}

std::chrono::year_month_weekday &f29(std::chrono::year_month_weekday &a0,
                                     const std::chrono::months &a1) {
  return a0.operator+=(a1);
}

std::chrono::year_month_weekday &f30(std::chrono::year_month_weekday &a0,
                                     const std::chrono::months &a1) {
  return a0.operator-=(a1);
}

std::chrono::year_month_weekday &f31(std::chrono::year_month_weekday &a0,
                                     const std::chrono::years &a1) {
  return a0.operator+=(a1);
}

std::chrono::year_month_weekday &f32(std::chrono::year_month_weekday &a0,
                                     const std::chrono::years &a1) {
  return a0.operator-=(a1);
}

std::chrono::year f33(const std::chrono::year_month_weekday &a0) {
  return a0.year();
}

std::chrono::month f34(const std::chrono::year_month_weekday &a0) {
  return a0.month();
}

std::chrono::weekday f35(const std::chrono::year_month_weekday &a0) {
  return a0.weekday();
}

unsigned int f36(const std::chrono::year_month_weekday &a0) {
  return a0.index();
}

std::chrono::weekday_indexed f37(const std::chrono::year_month_weekday &a0) {
  return a0.weekday_indexed();
}

std::chrono::sys_days f38(const std::chrono::year_month_weekday &a0) {
  return static_cast<std::chrono::sys_days>(a0);
}

std::chrono::local_days f39(const std::chrono::year_month_weekday &a0) {
  return static_cast<std::chrono::local_days>(a0);
}

bool f40(const std::chrono::year_month_weekday &a0) { return a0.ok(); }

std::chrono::year_month_weekday f41(const std::chrono::year_month_weekday &a0) {
  return std::chrono::year_month_weekday(a0);
}

std::chrono::year_month_weekday f42(std::chrono::year_month_weekday &&a0) {
  return std::chrono::year_month_weekday(std::move(a0));
}

std::chrono::year_month_weekday &
f43(std::chrono::year_month_weekday &a0,
    const std::chrono::year_month_weekday &a1) {
  return a0.operator=(a1);
}

std::chrono::year_month_weekday &f44(std::chrono::year_month_weekday &a0,
                                     std::chrono::year_month_weekday &&a1) {
  return a0.operator=(std::move(a1));
}

using t2 = std::chrono::year_month_weekday_last;

std::chrono::year_month_weekday_last f45(const std::chrono::year &a0,
                                         const std::chrono::month &a1,
                                         const std::chrono::weekday_last &a2) {
  return std::chrono::year_month_weekday_last(a0, a1, a2);
}

std::chrono::year_month_weekday_last &
f46(std::chrono::year_month_weekday_last &a0, const std::chrono::months &a1) {
  return a0.operator+=(a1);
}

std::chrono::year_month_weekday_last &
f47(std::chrono::year_month_weekday_last &a0, const std::chrono::months &a1) {
  return a0.operator-=(a1);
}

std::chrono::year_month_weekday_last &
f48(std::chrono::year_month_weekday_last &a0, const std::chrono::years &a1) {
  return a0.operator+=(a1);
}

std::chrono::year_month_weekday_last &
f49(std::chrono::year_month_weekday_last &a0, const std::chrono::years &a1) {
  return a0.operator-=(a1);
}

std::chrono::year f50(const std::chrono::year_month_weekday_last &a0) {
  return a0.year();
}

std::chrono::month f51(const std::chrono::year_month_weekday_last &a0) {
  return a0.month();
}

std::chrono::weekday f52(const std::chrono::year_month_weekday_last &a0) {
  return a0.weekday();
}

std::chrono::weekday_last f53(const std::chrono::year_month_weekday_last &a0) {
  return a0.weekday_last();
}

std::chrono::sys_days f54(const std::chrono::year_month_weekday_last &a0) {
  return static_cast<std::chrono::sys_days>(a0);
}

std::chrono::local_days f55(const std::chrono::year_month_weekday_last &a0) {
  return static_cast<std::chrono::local_days>(a0);
}

bool f56(const std::chrono::year_month_weekday_last &a0) { return a0.ok(); }

std::chrono::year_month_weekday_last
f57(const std::chrono::year_month_weekday_last &a0) {
  return std::chrono::year_month_weekday_last(a0);
}

std::chrono::year_month_weekday_last
f58(std::chrono::year_month_weekday_last &&a0) {
  return std::chrono::year_month_weekday_last(std::move(a0));
}

std::chrono::year_month_weekday_last &
f59(std::chrono::year_month_weekday_last &a0,
    const std::chrono::year_month_weekday_last &a1) {
  return a0.operator=(a1);
}

std::chrono::year_month_weekday_last &
f60(std::chrono::year_month_weekday_last &a0,
    std::chrono::year_month_weekday_last &&a1) {
  return a0.operator=(std::move(a1));
}
