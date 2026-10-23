#include <chrono>
#include <utility>

using t1 = std::chrono::month_weekday;

std::chrono::month_weekday f1(const std::chrono::month &a0,
                              const std::chrono::weekday_indexed &a1) {
  return std::chrono::month_weekday(a0, a1);
}

std::chrono::month f2(const std::chrono::month_weekday &a0) {
  return a0.month();
}

std::chrono::weekday_indexed f3(const std::chrono::month_weekday &a0) {
  return a0.weekday_indexed();
}

bool f4(const std::chrono::month_weekday &a0) { return a0.ok(); }

std::chrono::month_weekday f5(const std::chrono::month_weekday &a0) {
  return std::chrono::month_weekday(a0);
}

std::chrono::month_weekday f6(std::chrono::month_weekday &&a0) {
  return std::chrono::month_weekday(std::move(a0));
}

std::chrono::month_weekday &f7(std::chrono::month_weekday &a0,
                               const std::chrono::month_weekday &a1) {
  return a0.operator=(a1);
}

std::chrono::month_weekday &f8(std::chrono::month_weekday &a0,
                               std::chrono::month_weekday &&a1) {
  return a0.operator=(std::move(a1));
}

using t2 = std::chrono::month_weekday_last;

std::chrono::month_weekday_last f9(const std::chrono::month &a0,
                                   const std::chrono::weekday_last &a1) {
  return std::chrono::month_weekday_last(a0, a1);
}

std::chrono::month f10(const std::chrono::month_weekday_last &a0) {
  return a0.month();
}

std::chrono::weekday_last f11(const std::chrono::month_weekday_last &a0) {
  return a0.weekday_last();
}

bool f12(const std::chrono::month_weekday_last &a0) { return a0.ok(); }

std::chrono::month_weekday_last f13(const std::chrono::month_weekday_last &a0) {
  return std::chrono::month_weekday_last(a0);
}

std::chrono::month_weekday_last f14(std::chrono::month_weekday_last &&a0) {
  return std::chrono::month_weekday_last(std::move(a0));
}

std::chrono::month_weekday_last &
f15(std::chrono::month_weekday_last &a0,
    const std::chrono::month_weekday_last &a1) {
  return a0.operator=(a1);
}

std::chrono::month_weekday_last &f16(std::chrono::month_weekday_last &a0,
                                     std::chrono::month_weekday_last &&a1) {
  return a0.operator=(std::move(a1));
}

std::chrono::month_weekday f17(const std::chrono::month &a0,
                               const std::chrono::weekday_indexed &a1) {
  return a0 / a1;
}

std::chrono::month_weekday_last f18(const std::chrono::month &a0,
                                    const std::chrono::weekday_last &a1) {
  return a0 / a1;
}

std::chrono::month_weekday f19(const std::chrono::weekday_indexed &a0,
                               const std::chrono::month &a1) {
  return a0 / a1;
}

std::chrono::month_weekday f20(const std::chrono::weekday_indexed &a0, int a1) {
  return a0 / a1;
}

std::chrono::month_weekday_last f21(const std::chrono::weekday_last &a0,
                                    const std::chrono::month &a1) {
  return a0 / a1;
}

std::chrono::month_weekday_last f22(const std::chrono::weekday_last &a0,
                                    int a1) {
  return a0 / a1;
}

std::chrono::month_weekday f23(int a0, const std::chrono::weekday_indexed &a1) {
  return a0 / a1;
}

std::chrono::month_weekday_last f24(int a0,
                                    const std::chrono::weekday_last &a1) {
  return a0 / a1;
}

bool f25(const std::chrono::month_weekday &a0,
         const std::chrono::month_weekday &a1) {
  return a0 == a1;
}

bool f26(const std::chrono::month_weekday_last &a0,
         const std::chrono::month_weekday_last &a1) {
  return a0 == a1;
}
