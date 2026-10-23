#include <chrono>
#include <type_traits>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

using t1 = std::chrono::clock_time_conversion<std::chrono::system_clock,
                                              std::chrono::utc_clock>;

std::chrono::clock_time_conversion<std::chrono::system_clock,
                                   std::chrono::utc_clock>
f1() {
  return std::chrono::clock_time_conversion<std::chrono::system_clock,
                                            std::chrono::utc_clock>();
}

std::chrono::clock_time_conversion<std::chrono::system_clock,
                                   std::chrono::utc_clock>
f2(const std::chrono::clock_time_conversion<std::chrono::system_clock,
                                            std::chrono::utc_clock> &a0) {
  return std::chrono::clock_time_conversion<std::chrono::system_clock,
                                            std::chrono::utc_clock>(a0);
}

std::chrono::clock_time_conversion<std::chrono::system_clock,
                                   std::chrono::utc_clock>
f3(std::chrono::clock_time_conversion<std::chrono::system_clock,
                                      std::chrono::utc_clock> &&a0) {
  return std::chrono::clock_time_conversion<std::chrono::system_clock,
                                            std::chrono::utc_clock>(
      std::move(a0));
}

std::chrono::clock_time_conversion<std::chrono::system_clock,
                                   std::chrono::utc_clock> &
f4(std::chrono::clock_time_conversion<std::chrono::system_clock,
                                      std::chrono::utc_clock> &a0,
   const std::chrono::clock_time_conversion<std::chrono::system_clock,
                                            std::chrono::utc_clock> &a1) {
  return a0.operator=(a1);
}

std::chrono::clock_time_conversion<std::chrono::system_clock,
                                   std::chrono::utc_clock> &
f5(std::chrono::clock_time_conversion<std::chrono::system_clock,
                                      std::chrono::utc_clock> &a0,
   std::chrono::clock_time_conversion<std::chrono::system_clock,
                                      std::chrono::utc_clock> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
std::chrono::sys_time<std::common_type_t<T1, std::chrono::seconds>>
f6(const std::chrono::clock_time_conversion<std::chrono::system_clock,
                                            std::chrono::utc_clock> &a0,
   const std::chrono::utc_time<T1> &a1) {
  return a0.template operator()<T1>(a1);
}

using t2 = std::chrono::clock_time_conversion<std::chrono::system_clock,
                                              std::chrono::system_clock>;

std::chrono::clock_time_conversion<std::chrono::system_clock,
                                   std::chrono::system_clock>
f7() {
  return std::chrono::clock_time_conversion<std::chrono::system_clock,
                                            std::chrono::system_clock>();
}

std::chrono::clock_time_conversion<std::chrono::system_clock,
                                   std::chrono::system_clock>
f8(const std::chrono::clock_time_conversion<std::chrono::system_clock,
                                            std::chrono::system_clock> &a0) {
  return std::chrono::clock_time_conversion<std::chrono::system_clock,
                                            std::chrono::system_clock>(a0);
}

std::chrono::clock_time_conversion<std::chrono::system_clock,
                                   std::chrono::system_clock>
f9(std::chrono::clock_time_conversion<std::chrono::system_clock,
                                      std::chrono::system_clock> &&a0) {
  return std::chrono::clock_time_conversion<std::chrono::system_clock,
                                            std::chrono::system_clock>(
      std::move(a0));
}

std::chrono::clock_time_conversion<std::chrono::system_clock,
                                   std::chrono::system_clock> &
f10(std::chrono::clock_time_conversion<std::chrono::system_clock,
                                       std::chrono::system_clock> &a0,
    const std::chrono::clock_time_conversion<std::chrono::system_clock,
                                             std::chrono::system_clock> &a1) {
  return a0.operator=(a1);
}

std::chrono::clock_time_conversion<std::chrono::system_clock,
                                   std::chrono::system_clock> &
f11(std::chrono::clock_time_conversion<std::chrono::system_clock,
                                       std::chrono::system_clock> &a0,
    std::chrono::clock_time_conversion<std::chrono::system_clock,
                                       std::chrono::system_clock> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
std::chrono::sys_time<T1>
f12(const std::chrono::clock_time_conversion<std::chrono::system_clock,
                                             std::chrono::system_clock> &a0,
    const std::chrono::sys_time<T1> &a1) {
  return a0.template operator()<T1>(a1);
}

using t3 = std::chrono::clock_time_conversion<std::chrono::utc_clock,
                                              std::chrono::system_clock>;

std::chrono::clock_time_conversion<std::chrono::utc_clock,
                                   std::chrono::system_clock>
f13() {
  return std::chrono::clock_time_conversion<std::chrono::utc_clock,
                                            std::chrono::system_clock>();
}

std::chrono::clock_time_conversion<std::chrono::utc_clock,
                                   std::chrono::system_clock>
f14(const std::chrono::clock_time_conversion<std::chrono::utc_clock,
                                             std::chrono::system_clock> &a0) {
  return std::chrono::clock_time_conversion<std::chrono::utc_clock,
                                            std::chrono::system_clock>(a0);
}

std::chrono::clock_time_conversion<std::chrono::utc_clock,
                                   std::chrono::system_clock>
f15(std::chrono::clock_time_conversion<std::chrono::utc_clock,
                                       std::chrono::system_clock> &&a0) {
  return std::chrono::clock_time_conversion<std::chrono::utc_clock,
                                            std::chrono::system_clock>(
      std::move(a0));
}

std::chrono::clock_time_conversion<std::chrono::utc_clock,
                                   std::chrono::system_clock> &
f16(std::chrono::clock_time_conversion<std::chrono::utc_clock,
                                       std::chrono::system_clock> &a0,
    const std::chrono::clock_time_conversion<std::chrono::utc_clock,
                                             std::chrono::system_clock> &a1) {
  return a0.operator=(a1);
}

std::chrono::clock_time_conversion<std::chrono::utc_clock,
                                   std::chrono::system_clock> &
f17(std::chrono::clock_time_conversion<std::chrono::utc_clock,
                                       std::chrono::system_clock> &a0,
    std::chrono::clock_time_conversion<std::chrono::utc_clock,
                                       std::chrono::system_clock> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
std::chrono::utc_time<std::common_type_t<T1, std::chrono::seconds>>
f18(const std::chrono::clock_time_conversion<std::chrono::utc_clock,
                                             std::chrono::system_clock> &a0,
    const std::chrono::sys_time<T1> &a1) {
  return a0.template operator()<T1>(a1);
}

using t4 = std::chrono::clock_time_conversion<std::chrono::utc_clock,
                                              std::chrono::utc_clock>;

std::chrono::clock_time_conversion<std::chrono::utc_clock,
                                   std::chrono::utc_clock>
f19() {
  return std::chrono::clock_time_conversion<std::chrono::utc_clock,
                                            std::chrono::utc_clock>();
}

std::chrono::clock_time_conversion<std::chrono::utc_clock,
                                   std::chrono::utc_clock>
f20(const std::chrono::clock_time_conversion<std::chrono::utc_clock,
                                             std::chrono::utc_clock> &a0) {
  return std::chrono::clock_time_conversion<std::chrono::utc_clock,
                                            std::chrono::utc_clock>(a0);
}

std::chrono::clock_time_conversion<std::chrono::utc_clock,
                                   std::chrono::utc_clock>
f21(std::chrono::clock_time_conversion<std::chrono::utc_clock,
                                       std::chrono::utc_clock> &&a0) {
  return std::chrono::clock_time_conversion<std::chrono::utc_clock,
                                            std::chrono::utc_clock>(
      std::move(a0));
}

std::chrono::clock_time_conversion<std::chrono::utc_clock,
                                   std::chrono::utc_clock> &
f22(std::chrono::clock_time_conversion<std::chrono::utc_clock,
                                       std::chrono::utc_clock> &a0,
    const std::chrono::clock_time_conversion<std::chrono::utc_clock,
                                             std::chrono::utc_clock> &a1) {
  return a0.operator=(a1);
}

std::chrono::clock_time_conversion<std::chrono::utc_clock,
                                   std::chrono::utc_clock> &
f23(std::chrono::clock_time_conversion<std::chrono::utc_clock,
                                       std::chrono::utc_clock> &a0,
    std::chrono::clock_time_conversion<std::chrono::utc_clock,
                                       std::chrono::utc_clock> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
std::chrono::utc_time<T1>
f24(const std::chrono::clock_time_conversion<std::chrono::utc_clock,
                                             std::chrono::utc_clock> &a0,
    const std::chrono::utc_time<T1> &a1) {
  return a0.template operator()<T1>(a1);
}

template <typename T1>
using t5 = std::chrono::clock_time_conversion<std::chrono::system_clock, T1>;

template <typename T1, typename T2, typename T3 = T1>
auto f25(
    const std::chrono::clock_time_conversion<std::chrono::system_clock, T1> &a0,
    const std::chrono::time_point<T1, T2> &a1) {
  return a0.template operator()<T2>(a1);
}

template <typename T1>
using t6 = std::chrono::clock_time_conversion<std::chrono::utc_clock, T1>;

template <typename T1, typename T2, typename T3 = T1>
auto f26(
    const std::chrono::clock_time_conversion<std::chrono::utc_clock, T1> &a0,
    const std::chrono::time_point<T1, T2> &a1) {
  return a0.template operator()<T2>(a1);
}

template <typename T1>
using t7 = std::chrono::clock_time_conversion<T1, std::chrono::system_clock>;

template <typename T1, typename T2, typename T3 = T1>
auto f27(
    const std::chrono::clock_time_conversion<T1, std::chrono::system_clock> &a0,
    const std::chrono::sys_time<T2> &a1) {
  return a0.template operator()<T2>(a1);
}

template <typename T1>
using t8 = std::chrono::clock_time_conversion<T1, std::chrono::utc_clock>;

template <typename T1, typename T2, typename T3 = T1>
auto f28(
    const std::chrono::clock_time_conversion<T1, std::chrono::utc_clock> &a0,
    const std::chrono::utc_time<T2> &a1) {
  return a0.template operator()<T2>(a1);
}

template <typename T1> using t9 = std::chrono::clock_time_conversion<T1, T1>;

template <typename T1, typename T2>
std::chrono::time_point<T1, T2>
f29(const std::chrono::clock_time_conversion<T1, T1> &a0,
    const std::chrono::time_point<T1, T2> &a1) {
  return a0.template operator()<T2>(a1);
}

template <typename T1, typename T2>
using t10 = std::chrono::clock_time_conversion<T1, T2>;
