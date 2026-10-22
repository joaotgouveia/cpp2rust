#include "../common/init.h"
#include "../common/traits.h"
#include <chrono>
#include <string_view>
#include <utility>

using t1 = std::chrono::tzdb;

const std::chrono::time_zone *f1(const std::chrono::tzdb &a0,
                                 std::string_view a1) {
  return a0.locate_zone(std::move(a1));
}

const std::chrono::time_zone *f2(const std::chrono::tzdb &a0) {
  return a0.current_zone();
}

std::chrono::tzdb f3() { return std::chrono::tzdb(); }

std::chrono::tzdb f4(std::chrono::tzdb &&a0) {
  return std::chrono::tzdb(std::move(a0));
}

std::chrono::tzdb &f5(std::chrono::tzdb &a0, std::chrono::tzdb &&a1) {
  return a0.operator=(std::move(a1));
}
