#include "../common/init.h"
#include "../common/traits.h"
#include <chrono>
#include <compare>
#include <utility>

using t1 = std::chrono::month_day;

std::chrono::month_day f1() { return std::chrono::month_day(); }

std::chrono::month_day f2(const std::chrono::month &a0,
                          const std::chrono::day &a1) {
  return std::chrono::month_day(a0, a1);
}

std::chrono::month f3(const std::chrono::month_day &a0) { return a0.month(); }

std::chrono::day f4(const std::chrono::month_day &a0) { return a0.day(); }

bool f5(const std::chrono::month_day &a0) { return a0.ok(); }

std::chrono::month_day f6(const std::chrono::month_day &a0) {
  return std::chrono::month_day(a0);
}

std::chrono::month_day f7(std::chrono::month_day &&a0) {
  return std::chrono::month_day(std::move(a0));
}

std::chrono::month_day &f8(std::chrono::month_day &a0,
                           const std::chrono::month_day &a1) {
  return a0.operator=(a1);
}

std::chrono::month_day &f9(std::chrono::month_day &a0,
                           std::chrono::month_day &&a1) {
  return a0.operator=(std::move(a1));
}

using t2 = std::chrono::month_day_last;

std::chrono::month_day_last f10(const std::chrono::month &a0) {
  return std::chrono::month_day_last(a0);
}

std::chrono::month f11(const std::chrono::month_day_last &a0) {
  return a0.month();
}

bool f12(const std::chrono::month_day_last &a0) { return a0.ok(); }

std::chrono::month_day_last f13(const std::chrono::month_day_last &a0) {
  return std::chrono::month_day_last(a0);
}

std::chrono::month_day_last f14(std::chrono::month_day_last &&a0) {
  return std::chrono::month_day_last(std::move(a0));
}

std::chrono::month_day_last &f15(std::chrono::month_day_last &a0,
                                 const std::chrono::month_day_last &a1) {
  return a0.operator=(a1);
}

std::chrono::month_day_last &f16(std::chrono::month_day_last &a0,
                                 std::chrono::month_day_last &&a1) {
  return a0.operator=(std::move(a1));
}

std::chrono::month_day_last f17(std::chrono::last_spec a0,
                                const std::chrono::month &a1) {
  return std::move(a0) / a1;
}

std::chrono::month_day_last f18(std::chrono::last_spec a0, int a1) {
  return std::move(a0) / a1;
}

std::chrono::month_day f19(const std::chrono::day &a0,
                           const std::chrono::month &a1) {
  return a0 / a1;
}

std::chrono::month_day f20(const std::chrono::day &a0, int a1) {
  return a0 / a1;
}

std::chrono::month_day_last f21(const std::chrono::month &a0,
                                std::chrono::last_spec a1) {
  return a0 / std::move(a1);
}

std::chrono::month_day f22(const std::chrono::month &a0,
                           const std::chrono::day &a1) {
  return a0 / a1;
}

std::chrono::month_day f23(const std::chrono::month &a0, int a1) {
  return a0 / a1;
}

std::chrono::month_day_last f24(int a0, std::chrono::last_spec a1) {
  return a0 / std::move(a1);
}

std::chrono::month_day f25(int a0, const std::chrono::day &a1) {
  return a0 / a1;
}

std::strong_ordering f26(const std::chrono::month_day &a0,
                         const std::chrono::month_day &a1) {
  return a0 <=> a1;
}

std::strong_ordering f27(const std::chrono::month_day_last &a0,
                         const std::chrono::month_day_last &a1) {
  return a0 <=> a1;
}

bool f28(const std::chrono::month_day &a0, const std::chrono::month_day &a1) {
  return a0 == a1;
}

bool f29(const std::chrono::month_day_last &a0,
         const std::chrono::month_day_last &a1) {
  return a0 == a1;
}
