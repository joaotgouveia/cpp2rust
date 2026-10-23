#include <chrono>
#include <compare>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

std::chrono::year_month_day f1(const std::chrono::months &a0,
                               const std::chrono::year_month_day &a1) {
  return a0 + a1;
}

std::chrono::year_month_day_last
f2(const std::chrono::months &a0, const std::chrono::year_month_day_last &a1) {
  return a0 + a1;
}

std::chrono::year_month_day f3(const std::chrono::years &a0,
                               const std::chrono::year_month_day &a1) {
  return a0 + a1;
}

std::chrono::year_month_day_last
f4(const std::chrono::years &a0, const std::chrono::year_month_day_last &a1) {
  return a0 + a1;
}

std::chrono::year_month_day f5(const std::chrono::year_month_day &a0,
                               const std::chrono::months &a1) {
  return a0 + a1;
}

std::chrono::year_month_day f6(const std::chrono::year_month_day &a0,
                               const std::chrono::years &a1) {
  return a0 + a1;
}

std::chrono::year_month_day_last f7(const std::chrono::year_month_day_last &a0,
                                    const std::chrono::months &a1) {
  return a0 + a1;
}

std::chrono::year_month_day_last f8(const std::chrono::year_month_day_last &a0,
                                    const std::chrono::years &a1) {
  return a0 + a1;
}

std::chrono::year_month_day f9(const std::chrono::year_month_day &a0,
                               const std::chrono::months &a1) {
  return a0 - a1;
}

std::chrono::year_month_day f10(const std::chrono::year_month_day &a0,
                                const std::chrono::years &a1) {
  return a0 - a1;
}

std::chrono::year_month_day_last f11(const std::chrono::year_month_day_last &a0,
                                     const std::chrono::months &a1) {
  return a0 - a1;
}

std::chrono::year_month_day_last f12(const std::chrono::year_month_day_last &a0,
                                     const std::chrono::years &a1) {
  return a0 - a1;
}

std::chrono::year_month_day f13(const std::chrono::month_day &a0,
                                const std::chrono::year &a1) {
  return a0 / a1;
}

std::chrono::year_month_day f14(const std::chrono::month_day &a0, int a1) {
  return a0 / a1;
}

std::chrono::year_month_day_last f15(const std::chrono::month_day_last &a0,
                                     const std::chrono::year &a1) {
  return a0 / a1;
}

std::chrono::year_month_day_last f16(const std::chrono::month_day_last &a0,
                                     int a1) {
  return a0 / a1;
}

std::chrono::year_month_day f17(const std::chrono::year &a0,
                                const std::chrono::month_day &a1) {
  return a0 / a1;
}

std::chrono::year_month_day_last f18(const std::chrono::year &a0,
                                     const std::chrono::month_day_last &a1) {
  return a0 / a1;
}

std::chrono::year_month_day_last f19(const std::chrono::year_month &a0,
                                     std::chrono::last_spec a1) {
  return a0 / std::move(a1);
}

std::chrono::year_month_day f20(const std::chrono::year_month &a0,
                                const std::chrono::day &a1) {
  return a0 / a1;
}

std::chrono::year_month_day f21(const std::chrono::year_month &a0, int a1) {
  return a0 / a1;
}

std::chrono::year_month_day f22(int a0, const std::chrono::month_day &a1) {
  return a0 / a1;
}

std::chrono::year_month_day_last f23(int a0,
                                     const std::chrono::month_day_last &a1) {
  return a0 / a1;
}

std::strong_ordering f24(const std::chrono::year_month_day &a0,
                         const std::chrono::year_month_day &a1) {
  return a0 <=> a1;
}

std::strong_ordering f25(const std::chrono::year_month_day_last &a0,
                         const std::chrono::year_month_day_last &a1) {
  return a0 <=> a1;
}

bool f26(const std::chrono::year_month_day &a0,
         const std::chrono::year_month_day &a1) {
  return a0 == a1;
}

bool f27(const std::chrono::year_month_day_last &a0,
         const std::chrono::year_month_day_last &a1) {
  return a0 == a1;
}

using t1 = std::chrono::year_month_day;

std::chrono::year_month_day f28() { return std::chrono::year_month_day(); }

std::chrono::year_month_day f29(const std::chrono::year &a0,
                                const std::chrono::month &a1,
                                const std::chrono::day &a2) {
  return std::chrono::year_month_day(a0, a1, a2);
}

std::chrono::year_month_day f30(const std::chrono::year_month_day_last &a0) {
  return std::chrono::year_month_day(a0);
}

std::chrono::year_month_day f31(const std::chrono::sys_days &a0) {
  return std::chrono::year_month_day(a0);
}

std::chrono::year_month_day f32(const std::chrono::local_days &a0) {
  return std::chrono::year_month_day(a0);
}

std::chrono::year_month_day &f33(std::chrono::year_month_day &a0,
                                 const std::chrono::months &a1) {
  return a0.operator+=(a1);
}

std::chrono::year_month_day &f34(std::chrono::year_month_day &a0,
                                 const std::chrono::months &a1) {
  return a0.operator-=(a1);
}

std::chrono::year_month_day &f35(std::chrono::year_month_day &a0,
                                 const std::chrono::years &a1) {
  return a0.operator+=(a1);
}

std::chrono::year_month_day &f36(std::chrono::year_month_day &a0,
                                 const std::chrono::years &a1) {
  return a0.operator-=(a1);
}

std::chrono::year f37(const std::chrono::year_month_day &a0) {
  return a0.year();
}

std::chrono::month f38(const std::chrono::year_month_day &a0) {
  return a0.month();
}

std::chrono::day f39(const std::chrono::year_month_day &a0) { return a0.day(); }

std::chrono::sys_days f40(const std::chrono::year_month_day &a0) {
  return static_cast<std::chrono::sys_days>(a0);
}

std::chrono::local_days f41(const std::chrono::year_month_day &a0) {
  return static_cast<std::chrono::local_days>(a0);
}

bool f42(const std::chrono::year_month_day &a0) { return a0.ok(); }

std::chrono::year_month_day f43(const std::chrono::year_month_day &a0) {
  return std::chrono::year_month_day(a0);
}

std::chrono::year_month_day f44(std::chrono::year_month_day &&a0) {
  return std::chrono::year_month_day(std::move(a0));
}

std::chrono::year_month_day &f45(std::chrono::year_month_day &a0,
                                 const std::chrono::year_month_day &a1) {
  return a0.operator=(a1);
}

std::chrono::year_month_day &f46(std::chrono::year_month_day &a0,
                                 std::chrono::year_month_day &&a1) {
  return a0.operator=(std::move(a1));
}

using t2 = std::chrono::year_month_day_last;

std::chrono::year_month_day_last f47(const std::chrono::year &a0,
                                     const std::chrono::month_day_last &a1) {
  return std::chrono::year_month_day_last(a0, a1);
}

std::chrono::year_month_day_last &f48(std::chrono::year_month_day_last &a0,
                                      const std::chrono::months &a1) {
  return a0.operator+=(a1);
}

std::chrono::year_month_day_last &f49(std::chrono::year_month_day_last &a0,
                                      const std::chrono::months &a1) {
  return a0.operator-=(a1);
}

std::chrono::year_month_day_last &f50(std::chrono::year_month_day_last &a0,
                                      const std::chrono::years &a1) {
  return a0.operator+=(a1);
}

std::chrono::year_month_day_last &f51(std::chrono::year_month_day_last &a0,
                                      const std::chrono::years &a1) {
  return a0.operator-=(a1);
}

std::chrono::year f52(const std::chrono::year_month_day_last &a0) {
  return a0.year();
}

std::chrono::month f53(const std::chrono::year_month_day_last &a0) {
  return a0.month();
}

std::chrono::month_day_last f54(const std::chrono::year_month_day_last &a0) {
  return a0.month_day_last();
}

std::chrono::day f55(const std::chrono::year_month_day_last &a0) {
  return a0.day();
}

std::chrono::sys_days f56(const std::chrono::year_month_day_last &a0) {
  return static_cast<std::chrono::sys_days>(a0);
}

std::chrono::local_days f57(const std::chrono::year_month_day_last &a0) {
  return static_cast<std::chrono::local_days>(a0);
}

bool f58(const std::chrono::year_month_day_last &a0) { return a0.ok(); }

std::chrono::year_month_day_last
f59(const std::chrono::year_month_day_last &a0) {
  return std::chrono::year_month_day_last(a0);
}

std::chrono::year_month_day_last f60(std::chrono::year_month_day_last &&a0) {
  return std::chrono::year_month_day_last(std::move(a0));
}

std::chrono::year_month_day_last &
f61(std::chrono::year_month_day_last &a0,
    const std::chrono::year_month_day_last &a1) {
  return a0.operator=(a1);
}

std::chrono::year_month_day_last &f62(std::chrono::year_month_day_last &a0,
                                      std::chrono::year_month_day_last &&a1) {
  return a0.operator=(std::move(a1));
}
