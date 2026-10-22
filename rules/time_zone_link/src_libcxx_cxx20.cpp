#include "../common/init.h"
#include "../common/traits.h"
#include <chrono>
#include <compare>
#include <string_view>
#include <utility>

std::strong_ordering f1(const std::chrono::time_zone_link &a0,
                        const std::chrono::time_zone_link &a1) {
  return a0 <=> a1;
}

bool f2(const std::chrono::time_zone_link &a0,
        const std::chrono::time_zone_link &a1) {
  return a0 == a1;
}

using t1 = std::chrono::time_zone_link;

std::chrono::time_zone_link f3(std::chrono::time_zone_link &&a0) {
  return std::chrono::time_zone_link(std::move(a0));
}

std::chrono::time_zone_link &f4(std::chrono::time_zone_link &a0,
                                std::chrono::time_zone_link &&a1) {
  return a0.operator=(std::move(a1));
}

std::string_view f5(const std::chrono::time_zone_link &a0) { return a0.name(); }

std::string_view f6(const std::chrono::time_zone_link &a0) {
  return a0.target();
}
