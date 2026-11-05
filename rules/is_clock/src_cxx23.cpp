#include <chrono>
#include <type_traits>
#include <utility>

using t1 = std::chrono::is_clock<std::chrono::steady_clock>;

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

bool f8() { return std::chrono::is_clock<std::chrono::steady_clock>::value; }

using t2 = std::chrono::is_clock<std::chrono::system_clock>;

std::chrono::is_clock<std::chrono::system_clock> f9() {
  return std::chrono::is_clock<std::chrono::system_clock>();
}

std::chrono::is_clock<std::chrono::system_clock>
f10(const std::chrono::is_clock<std::chrono::system_clock> &a0) {
  return std::chrono::is_clock<std::chrono::system_clock>(a0);
}

std::chrono::is_clock<std::chrono::system_clock>
f11(std::chrono::is_clock<std::chrono::system_clock> &&a0) {
  return std::chrono::is_clock<std::chrono::system_clock>(std::move(a0));
}

std::chrono::is_clock<std::chrono::system_clock> &
f12(std::chrono::is_clock<std::chrono::system_clock> &a0,
    const std::chrono::is_clock<std::chrono::system_clock> &a1) {
  return a0.operator=(a1);
}

std::chrono::is_clock<std::chrono::system_clock> &
f13(std::chrono::is_clock<std::chrono::system_clock> &a0,
    std::chrono::is_clock<std::chrono::system_clock> &&a1) {
  return a0.operator=(std::move(a1));
}

std::integral_constant<bool, true>::value_type
f14(const std::chrono::is_clock<std::chrono::system_clock> &a0) {
  return static_cast<std::integral_constant<bool, true>::value_type>(a0);
}

std::integral_constant<bool, true>::value_type
f15(const std::chrono::is_clock<std::chrono::system_clock> &a0) {
  return a0.operator()();
}

bool f16() { return std::chrono::is_clock<std::chrono::system_clock>::value; }

using t3 = std::chrono::is_clock<std::chrono::gps_clock>;

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

bool f24() { return std::chrono::is_clock<std::chrono::gps_clock>::value; }

using t4 = std::chrono::is_clock<std::chrono::tai_clock>;

std::chrono::is_clock<std::chrono::tai_clock> f25() {
  return std::chrono::is_clock<std::chrono::tai_clock>();
}

std::chrono::is_clock<std::chrono::tai_clock>
f26(const std::chrono::is_clock<std::chrono::tai_clock> &a0) {
  return std::chrono::is_clock<std::chrono::tai_clock>(a0);
}

std::chrono::is_clock<std::chrono::tai_clock>
f27(std::chrono::is_clock<std::chrono::tai_clock> &&a0) {
  return std::chrono::is_clock<std::chrono::tai_clock>(std::move(a0));
}

std::chrono::is_clock<std::chrono::tai_clock> &
f28(std::chrono::is_clock<std::chrono::tai_clock> &a0,
    const std::chrono::is_clock<std::chrono::tai_clock> &a1) {
  return a0.operator=(a1);
}

std::chrono::is_clock<std::chrono::tai_clock> &
f29(std::chrono::is_clock<std::chrono::tai_clock> &a0,
    std::chrono::is_clock<std::chrono::tai_clock> &&a1) {
  return a0.operator=(std::move(a1));
}

std::integral_constant<bool, true>::value_type
f30(const std::chrono::is_clock<std::chrono::tai_clock> &a0) {
  return static_cast<std::integral_constant<bool, true>::value_type>(a0);
}

std::integral_constant<bool, true>::value_type
f31(const std::chrono::is_clock<std::chrono::tai_clock> &a0) {
  return a0.operator()();
}

bool f32() { return std::chrono::is_clock<std::chrono::tai_clock>::value; }

using t5 = std::chrono::is_clock<std::chrono::utc_clock>;

std::chrono::is_clock<std::chrono::utc_clock> f33() {
  return std::chrono::is_clock<std::chrono::utc_clock>();
}

std::chrono::is_clock<std::chrono::utc_clock>
f34(const std::chrono::is_clock<std::chrono::utc_clock> &a0) {
  return std::chrono::is_clock<std::chrono::utc_clock>(a0);
}

std::chrono::is_clock<std::chrono::utc_clock>
f35(std::chrono::is_clock<std::chrono::utc_clock> &&a0) {
  return std::chrono::is_clock<std::chrono::utc_clock>(std::move(a0));
}

std::chrono::is_clock<std::chrono::utc_clock> &
f36(std::chrono::is_clock<std::chrono::utc_clock> &a0,
    const std::chrono::is_clock<std::chrono::utc_clock> &a1) {
  return a0.operator=(a1);
}

std::chrono::is_clock<std::chrono::utc_clock> &
f37(std::chrono::is_clock<std::chrono::utc_clock> &a0,
    std::chrono::is_clock<std::chrono::utc_clock> &&a1) {
  return a0.operator=(std::move(a1));
}

std::integral_constant<bool, true>::value_type
f38(const std::chrono::is_clock<std::chrono::utc_clock> &a0) {
  return static_cast<std::integral_constant<bool, true>::value_type>(a0);
}

std::integral_constant<bool, true>::value_type
f39(const std::chrono::is_clock<std::chrono::utc_clock> &a0) {
  return a0.operator()();
}

bool f40() { return std::chrono::is_clock<std::chrono::utc_clock>::value; }

template <typename T1> using t6 = std::chrono::is_clock<T1>;

template <typename T1> bool f41(const std::chrono::is_clock<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1> bool f42(const std::chrono::is_clock<T1> &a0) {
  return a0.operator()();
}

template <typename T1> bool f43() { return std::chrono::is_clock<T1>::value; }
