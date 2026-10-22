#include "../common/init.h"
#include "../common/traits.h"
#include <chrono>
#include <compare>
#include <string_view>
#include <type_traits>
#include <utility>

using t1 = std::chrono::choose;

std::chrono::choose f1() { return std::chrono::choose::earliest; }

std::chrono::choose f2() { return std::chrono::choose::latest; }

std::strong_ordering f3(const std::chrono::time_zone &a0,
                        const std::chrono::time_zone &a1) {
  return a0 <=> a1;
}

bool f4(const std::chrono::time_zone &a0, const std::chrono::time_zone &a1) {
  return a0 == a1;
}

using t2 = std::chrono::time_zone;

std::string_view f5(const std::chrono::time_zone &a0) { return a0.name(); }

template <typename T1>
std::chrono::sys_info f6(const std::chrono::time_zone &a0,
                         const std::chrono::sys_time<T1> &a1) {
  return a0.template get_info<T1>(a1);
}

template <typename T1>
std::chrono::local_info f7(const std::chrono::time_zone &a0,
                           const std::chrono::local_time<T1> &a1) {
  return a0.template get_info<T1>(a1);
}

template <typename T1>
std::chrono::sys_time<std::common_type_t<T1, std::chrono::seconds>>
f8(const std::chrono::time_zone &a0, const std::chrono::local_time<T1> &a1) {
  return a0.template to_sys<T1>(a1);
}

template <typename T1>
std::chrono::sys_time<std::common_type_t<T1, std::chrono::seconds>>
f9(const std::chrono::time_zone &a0, const std::chrono::local_time<T1> &a1,
   std::chrono::choose a2) {
  return a0.template to_sys<T1>(a1, a2);
}

template <typename T1>
std::chrono::local_time<std::common_type_t<T1, std::chrono::seconds>>
f10(const std::chrono::time_zone &a0, const std::chrono::sys_time<T1> &a1) {
  return a0.template to_local<T1>(a1);
}
