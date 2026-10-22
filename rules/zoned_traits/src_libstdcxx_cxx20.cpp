#include "../common/init.h"
#include "../common/traits.h"
#include <chrono>
#include <string_view>
#include <utility>

using t1 = std::chrono::zoned_traits<const std::chrono::time_zone *>;

const std::chrono::time_zone *f1() {
  return std::chrono::zoned_traits<
      const std::chrono::time_zone *>::default_zone();
}

const std::chrono::time_zone *f2(std::string_view a0) {
  return std::chrono::zoned_traits<const std::chrono::time_zone *>::locate_zone(
      std::move(a0));
}

std::chrono::zoned_traits<const std::chrono::time_zone *> f3() {
  return std::chrono::zoned_traits<const std::chrono::time_zone *>();
}

std::chrono::zoned_traits<const std::chrono::time_zone *>
f4(const std::chrono::zoned_traits<const std::chrono::time_zone *> &a0) {
  return std::chrono::zoned_traits<const std::chrono::time_zone *>(a0);
}

std::chrono::zoned_traits<const std::chrono::time_zone *>
f5(std::chrono::zoned_traits<const std::chrono::time_zone *> &&a0) {
  return std::chrono::zoned_traits<const std::chrono::time_zone *>(
      std::move(a0));
}

std::chrono::zoned_traits<const std::chrono::time_zone *> &
f6(std::chrono::zoned_traits<const std::chrono::time_zone *> &a0,
   const std::chrono::zoned_traits<const std::chrono::time_zone *> &a1) {
  return a0.operator=(a1);
}

std::chrono::zoned_traits<const std::chrono::time_zone *> &
f7(std::chrono::zoned_traits<const std::chrono::time_zone *> &a0,
   std::chrono::zoned_traits<const std::chrono::time_zone *> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> using t2 = std::chrono::zoned_traits<T1>;
