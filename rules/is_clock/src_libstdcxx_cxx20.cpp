#include <chrono>
#include <type_traits>
#include <utility>

using t2 = std::chrono::is_clock<std::chrono::steady_clock>;

std::chrono::is_clock<std::chrono::steady_clock> f1() {
  return std::chrono::is_clock<std::chrono::steady_clock>();
}

std::chrono::is_clock<std::chrono::steady_clock>
f2(const std::chrono::is_clock<std::chrono::steady_clock> &a0) {
  return std::chrono::is_clock<std::chrono::steady_clock>(a0);
}

std::chrono::is_clock<std::chrono::steady_clock>
f3(std::chrono::is_clock<std::chrono::steady_clock> &&a0) {
  return std::chrono::is_clock<std::chrono::steady_clock>(std::move(a0));
}

std::chrono::is_clock<std::chrono::steady_clock> &
f4(std::chrono::is_clock<std::chrono::steady_clock> &a0,
   const std::chrono::is_clock<std::chrono::steady_clock> &a1) {
  return a0.operator=(a1);
}

std::chrono::is_clock<std::chrono::steady_clock> &
f5(std::chrono::is_clock<std::chrono::steady_clock> &a0,
   std::chrono::is_clock<std::chrono::steady_clock> &&a1) {
  return a0.operator=(std::move(a1));
}

std::integral_constant<bool, true>::value_type
f6(const std::chrono::is_clock<std::chrono::steady_clock> &a0) {
  return static_cast<std::integral_constant<bool, true>::value_type>(a0);
}

std::integral_constant<bool, true>::value_type
f7(const std::chrono::is_clock<std::chrono::steady_clock> &a0) {
  return a0.operator()();
}

using t3 = std::chrono::is_clock<std::chrono::system_clock>;

std::chrono::is_clock<std::chrono::system_clock> f8() {
  return std::chrono::is_clock<std::chrono::system_clock>();
}

std::chrono::is_clock<std::chrono::system_clock>
f9(const std::chrono::is_clock<std::chrono::system_clock> &a0) {
  return std::chrono::is_clock<std::chrono::system_clock>(a0);
}

std::chrono::is_clock<std::chrono::system_clock>
f10(std::chrono::is_clock<std::chrono::system_clock> &&a0) {
  return std::chrono::is_clock<std::chrono::system_clock>(std::move(a0));
}

std::chrono::is_clock<std::chrono::system_clock> &
f11(std::chrono::is_clock<std::chrono::system_clock> &a0,
    const std::chrono::is_clock<std::chrono::system_clock> &a1) {
  return a0.operator=(a1);
}

std::chrono::is_clock<std::chrono::system_clock> &
f12(std::chrono::is_clock<std::chrono::system_clock> &a0,
    std::chrono::is_clock<std::chrono::system_clock> &&a1) {
  return a0.operator=(std::move(a1));
}

std::integral_constant<bool, true>::value_type
f13(const std::chrono::is_clock<std::chrono::system_clock> &a0) {
  return static_cast<std::integral_constant<bool, true>::value_type>(a0);
}

std::integral_constant<bool, true>::value_type
f14(const std::chrono::is_clock<std::chrono::system_clock> &a0) {
  return a0.operator()();
}

using t4 = std::chrono::is_clock<std::chrono::gps_clock>;

std::chrono::is_clock<std::chrono::gps_clock> f15() {
  return std::chrono::is_clock<std::chrono::gps_clock>();
}

std::chrono::is_clock<std::chrono::gps_clock>
f16(const std::chrono::is_clock<std::chrono::gps_clock> &a0) {
  return std::chrono::is_clock<std::chrono::gps_clock>(a0);
}

std::chrono::is_clock<std::chrono::gps_clock>
f17(std::chrono::is_clock<std::chrono::gps_clock> &&a0) {
  return std::chrono::is_clock<std::chrono::gps_clock>(std::move(a0));
}

std::chrono::is_clock<std::chrono::gps_clock> &
f18(std::chrono::is_clock<std::chrono::gps_clock> &a0,
    const std::chrono::is_clock<std::chrono::gps_clock> &a1) {
  return a0.operator=(a1);
}

std::chrono::is_clock<std::chrono::gps_clock> &
f19(std::chrono::is_clock<std::chrono::gps_clock> &a0,
    std::chrono::is_clock<std::chrono::gps_clock> &&a1) {
  return a0.operator=(std::move(a1));
}

std::integral_constant<bool, true>::value_type
f20(const std::chrono::is_clock<std::chrono::gps_clock> &a0) {
  return static_cast<std::integral_constant<bool, true>::value_type>(a0);
}

std::integral_constant<bool, true>::value_type
f21(const std::chrono::is_clock<std::chrono::gps_clock> &a0) {
  return a0.operator()();
}

using t5 = std::chrono::is_clock<std::chrono::tai_clock>;

std::chrono::is_clock<std::chrono::tai_clock> f22() {
  return std::chrono::is_clock<std::chrono::tai_clock>();
}

std::chrono::is_clock<std::chrono::tai_clock>
f23(const std::chrono::is_clock<std::chrono::tai_clock> &a0) {
  return std::chrono::is_clock<std::chrono::tai_clock>(a0);
}

std::chrono::is_clock<std::chrono::tai_clock>
f24(std::chrono::is_clock<std::chrono::tai_clock> &&a0) {
  return std::chrono::is_clock<std::chrono::tai_clock>(std::move(a0));
}

std::chrono::is_clock<std::chrono::tai_clock> &
f25(std::chrono::is_clock<std::chrono::tai_clock> &a0,
    const std::chrono::is_clock<std::chrono::tai_clock> &a1) {
  return a0.operator=(a1);
}

std::chrono::is_clock<std::chrono::tai_clock> &
f26(std::chrono::is_clock<std::chrono::tai_clock> &a0,
    std::chrono::is_clock<std::chrono::tai_clock> &&a1) {
  return a0.operator=(std::move(a1));
}

std::integral_constant<bool, true>::value_type
f27(const std::chrono::is_clock<std::chrono::tai_clock> &a0) {
  return static_cast<std::integral_constant<bool, true>::value_type>(a0);
}

std::integral_constant<bool, true>::value_type
f28(const std::chrono::is_clock<std::chrono::tai_clock> &a0) {
  return a0.operator()();
}

using t6 = std::chrono::is_clock<std::chrono::utc_clock>;

std::chrono::is_clock<std::chrono::utc_clock> f29() {
  return std::chrono::is_clock<std::chrono::utc_clock>();
}

std::chrono::is_clock<std::chrono::utc_clock>
f30(const std::chrono::is_clock<std::chrono::utc_clock> &a0) {
  return std::chrono::is_clock<std::chrono::utc_clock>(a0);
}

std::chrono::is_clock<std::chrono::utc_clock>
f31(std::chrono::is_clock<std::chrono::utc_clock> &&a0) {
  return std::chrono::is_clock<std::chrono::utc_clock>(std::move(a0));
}

std::chrono::is_clock<std::chrono::utc_clock> &
f32(std::chrono::is_clock<std::chrono::utc_clock> &a0,
    const std::chrono::is_clock<std::chrono::utc_clock> &a1) {
  return a0.operator=(a1);
}

std::chrono::is_clock<std::chrono::utc_clock> &
f33(std::chrono::is_clock<std::chrono::utc_clock> &a0,
    std::chrono::is_clock<std::chrono::utc_clock> &&a1) {
  return a0.operator=(std::move(a1));
}

std::integral_constant<bool, true>::value_type
f34(const std::chrono::is_clock<std::chrono::utc_clock> &a0) {
  return static_cast<std::integral_constant<bool, true>::value_type>(a0);
}

std::integral_constant<bool, true>::value_type
f35(const std::chrono::is_clock<std::chrono::utc_clock> &a0) {
  return a0.operator()();
}
