#include "../common/init.h"
#include "../common/traits.h"
#include <chrono>
#include <compare>
#include <utility>

using t1 = std::chrono::leap_second;

std::chrono::leap_second f1(const std::chrono::leap_second &a0) {
  return std::chrono::leap_second(a0);
}

std::chrono::leap_second &f2(std::chrono::leap_second &a0,
                             const std::chrono::leap_second &a1) {
  return a0.operator=(a1);
}

std::chrono::sys_seconds f3(const std::chrono::leap_second &a0) {
  return a0.date();
}

std::chrono::seconds f4(const std::chrono::leap_second &a0) {
  return a0.value();
}

std::strong_ordering f5(const std::chrono::leap_second &a0,
                        const std::chrono::leap_second &a1) {
  return a0 <=> a1;
}

bool f6(const std::chrono::leap_second &a0,
        const std::chrono::leap_second &a1) {
  return a0 == a1;
}
