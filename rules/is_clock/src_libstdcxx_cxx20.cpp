#include <chrono>
#include <type_traits>
#include <utility>

using t2 = std::chrono::is_clock<std::chrono::steady_clock>;

std::chrono::is_clock<std::chrono::steady_clock> f3() {
  return std::chrono::is_clock<std::chrono::steady_clock>();
}

std::chrono::is_clock<std::chrono::steady_clock>
f4(const std::chrono::is_clock<std::chrono::steady_clock> &a0) {
  return std::chrono::is_clock<std::chrono::steady_clock>(a0);
}

std::chrono::is_clock<std::chrono::steady_clock>
f5(std::chrono::is_clock<std::chrono::steady_clock> &&a0) {
  return std::chrono::is_clock<std::chrono::steady_clock>(std::move(a0));
}

std::chrono::is_clock<std::chrono::steady_clock> &
f6(std::chrono::is_clock<std::chrono::steady_clock> &a0,
   const std::chrono::is_clock<std::chrono::steady_clock> &a1) {
  return a0.operator=(a1);
}

std::chrono::is_clock<std::chrono::steady_clock> &
f7(std::chrono::is_clock<std::chrono::steady_clock> &a0,
   std::chrono::is_clock<std::chrono::steady_clock> &&a1) {
  return a0.operator=(std::move(a1));
}

std::integral_constant<bool, true>::value_type
f8(const std::chrono::is_clock<std::chrono::steady_clock> &a0) {
  return static_cast<std::integral_constant<bool, true>::value_type>(a0);
}

std::integral_constant<bool, true>::value_type
f9(const std::chrono::is_clock<std::chrono::steady_clock> &a0) {
  return a0.operator()();
}

using t3 = std::chrono::is_clock<std::chrono::system_clock>;

std::chrono::is_clock<std::chrono::system_clock> f10() {
  return std::chrono::is_clock<std::chrono::system_clock>();
}

std::chrono::is_clock<std::chrono::system_clock>
f11(const std::chrono::is_clock<std::chrono::system_clock> &a0) {
  return std::chrono::is_clock<std::chrono::system_clock>(a0);
}

std::chrono::is_clock<std::chrono::system_clock>
f12(std::chrono::is_clock<std::chrono::system_clock> &&a0) {
  return std::chrono::is_clock<std::chrono::system_clock>(std::move(a0));
}

std::chrono::is_clock<std::chrono::system_clock> &
f13(std::chrono::is_clock<std::chrono::system_clock> &a0,
    const std::chrono::is_clock<std::chrono::system_clock> &a1) {
  return a0.operator=(a1);
}

std::chrono::is_clock<std::chrono::system_clock> &
f14(std::chrono::is_clock<std::chrono::system_clock> &a0,
    std::chrono::is_clock<std::chrono::system_clock> &&a1) {
  return a0.operator=(std::move(a1));
}

std::integral_constant<bool, true>::value_type
f15(const std::chrono::is_clock<std::chrono::system_clock> &a0) {
  return static_cast<std::integral_constant<bool, true>::value_type>(a0);
}

std::integral_constant<bool, true>::value_type
f16(const std::chrono::is_clock<std::chrono::system_clock> &a0) {
  return a0.operator()();
}

using t4 = std::chrono::is_clock<std::chrono::gps_clock>;

std::chrono::is_clock<std::chrono::gps_clock> f17() {
  return std::chrono::is_clock<std::chrono::gps_clock>();
}

std::chrono::is_clock<std::chrono::gps_clock>
f18(const std::chrono::is_clock<std::chrono::gps_clock> &a0) {
  return std::chrono::is_clock<std::chrono::gps_clock>(a0);
}

std::chrono::is_clock<std::chrono::gps_clock>
f19(std::chrono::is_clock<std::chrono::gps_clock> &&a0) {
  return std::chrono::is_clock<std::chrono::gps_clock>(std::move(a0));
}

std::chrono::is_clock<std::chrono::gps_clock> &
f20(std::chrono::is_clock<std::chrono::gps_clock> &a0,
    const std::chrono::is_clock<std::chrono::gps_clock> &a1) {
  return a0.operator=(a1);
}

std::chrono::is_clock<std::chrono::gps_clock> &
f21(std::chrono::is_clock<std::chrono::gps_clock> &a0,
    std::chrono::is_clock<std::chrono::gps_clock> &&a1) {
  return a0.operator=(std::move(a1));
}

std::integral_constant<bool, true>::value_type
f22(const std::chrono::is_clock<std::chrono::gps_clock> &a0) {
  return static_cast<std::integral_constant<bool, true>::value_type>(a0);
}

std::integral_constant<bool, true>::value_type
f23(const std::chrono::is_clock<std::chrono::gps_clock> &a0) {
  return a0.operator()();
}

using t5 = std::chrono::is_clock<std::chrono::tai_clock>;

std::chrono::is_clock<std::chrono::tai_clock> f24() {
  return std::chrono::is_clock<std::chrono::tai_clock>();
}

std::chrono::is_clock<std::chrono::tai_clock>
f25(const std::chrono::is_clock<std::chrono::tai_clock> &a0) {
  return std::chrono::is_clock<std::chrono::tai_clock>(a0);
}

std::chrono::is_clock<std::chrono::tai_clock>
f26(std::chrono::is_clock<std::chrono::tai_clock> &&a0) {
  return std::chrono::is_clock<std::chrono::tai_clock>(std::move(a0));
}

std::chrono::is_clock<std::chrono::tai_clock> &
f27(std::chrono::is_clock<std::chrono::tai_clock> &a0,
    const std::chrono::is_clock<std::chrono::tai_clock> &a1) {
  return a0.operator=(a1);
}

std::chrono::is_clock<std::chrono::tai_clock> &
f28(std::chrono::is_clock<std::chrono::tai_clock> &a0,
    std::chrono::is_clock<std::chrono::tai_clock> &&a1) {
  return a0.operator=(std::move(a1));
}

std::integral_constant<bool, true>::value_type
f29(const std::chrono::is_clock<std::chrono::tai_clock> &a0) {
  return static_cast<std::integral_constant<bool, true>::value_type>(a0);
}

std::integral_constant<bool, true>::value_type
f30(const std::chrono::is_clock<std::chrono::tai_clock> &a0) {
  return a0.operator()();
}

using t6 = std::chrono::is_clock<std::chrono::utc_clock>;

std::chrono::is_clock<std::chrono::utc_clock> f31() {
  return std::chrono::is_clock<std::chrono::utc_clock>();
}

std::chrono::is_clock<std::chrono::utc_clock>
f32(const std::chrono::is_clock<std::chrono::utc_clock> &a0) {
  return std::chrono::is_clock<std::chrono::utc_clock>(a0);
}

std::chrono::is_clock<std::chrono::utc_clock>
f33(std::chrono::is_clock<std::chrono::utc_clock> &&a0) {
  return std::chrono::is_clock<std::chrono::utc_clock>(std::move(a0));
}

std::chrono::is_clock<std::chrono::utc_clock> &
f34(std::chrono::is_clock<std::chrono::utc_clock> &a0,
    const std::chrono::is_clock<std::chrono::utc_clock> &a1) {
  return a0.operator=(a1);
}

std::chrono::is_clock<std::chrono::utc_clock> &
f35(std::chrono::is_clock<std::chrono::utc_clock> &a0,
    std::chrono::is_clock<std::chrono::utc_clock> &&a1) {
  return a0.operator=(std::move(a1));
}

std::integral_constant<bool, true>::value_type
f36(const std::chrono::is_clock<std::chrono::utc_clock> &a0) {
  return static_cast<std::integral_constant<bool, true>::value_type>(a0);
}

std::integral_constant<bool, true>::value_type
f37(const std::chrono::is_clock<std::chrono::utc_clock> &a0) {
  return a0.operator()();
}
