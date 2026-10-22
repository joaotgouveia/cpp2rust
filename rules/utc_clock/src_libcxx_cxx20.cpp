#include "../common/init.h"
#include "../common/traits.h"
#include <chrono>
#include <ratio>
#include <type_traits>
#include <utility>

template <typename T1>
std::chrono::leap_second_info f1(const std::chrono::utc_time<T1> &a0) {
  return std::chrono::get_leap_second_info<T1>(a0);
}

using t1 = std::chrono::leap_second_info;

std::chrono::leap_second_info f2() { return std::chrono::leap_second_info(); }

std::chrono::leap_second_info f3(const std::chrono::leap_second_info &a0) {
  return std::chrono::leap_second_info(a0);
}

std::chrono::leap_second_info f4(std::chrono::leap_second_info &&a0) {
  return std::chrono::leap_second_info(std::move(a0));
}

std::chrono::leap_second_info &f5(std::chrono::leap_second_info &a0,
                                  const std::chrono::leap_second_info &a1) {
  return a0.operator=(a1);
}

std::chrono::leap_second_info &f6(std::chrono::leap_second_info &a0,
                                  std::chrono::leap_second_info &&a1) {
  return a0.operator=(std::move(a1));
}

using t2 = std::chrono::utc_clock;

using t3 = std::chrono::utc_clock::rep;

using t4 = std::chrono::utc_clock::period;

using t5 = std::chrono::utc_clock::duration;

using t6 = std::chrono::utc_clock::time_point;

std::chrono::utc_clock::time_point f7() {
  return std::chrono::utc_clock::now();
}

std::chrono::utc_clock f8() { return std::chrono::utc_clock(); }

std::chrono::utc_clock f9(const std::chrono::utc_clock &a0) {
  return std::chrono::utc_clock(a0);
}

std::chrono::utc_clock f10(std::chrono::utc_clock &&a0) {
  return std::chrono::utc_clock(std::move(a0));
}

std::chrono::utc_clock &f11(std::chrono::utc_clock &a0,
                            const std::chrono::utc_clock &a1) {
  return a0.operator=(a1);
}

std::chrono::utc_clock &f12(std::chrono::utc_clock &a0,
                            std::chrono::utc_clock &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
std::chrono::sys_time<std::common_type_t<T1, std::chrono::seconds>>
f13(const std::chrono::utc_time<T1> &a0) {
  return std::chrono::utc_clock::to_sys(a0);
}

template <typename T1>
std::chrono::utc_time<std::common_type_t<T1, std::chrono::seconds>>
f14(const std::chrono::sys_time<T1> &a0) {
  return std::chrono::utc_clock::from_sys(a0);
}

using t7 = std::ratio<1, 1000000>::type;

using t8 = std::chrono::duration<long long, std::ratio<1, 1000000>>::rep;

using t9 = std::chrono::duration<long long, std::ratio<1, 1000000>>::period;

std::chrono::utc_clock::duration f15() {
  return std::chrono::utc_clock::duration();
}

std::chrono::duration<long long, std::ratio<1, 1000000>>::rep
f16(const std::chrono::utc_clock::duration &a0) {
  return a0.count();
}

typename std::common_type<std::chrono::utc_clock::duration>::type
f17(const std::chrono::utc_clock::duration &a0) {
  return a0.operator+();
}

typename std::common_type<std::chrono::utc_clock::duration>::type
f18(const std::chrono::utc_clock::duration &a0) {
  return a0.operator-();
}

std::chrono::utc_clock::duration &f19(std::chrono::utc_clock::duration &a0) {
  return a0.operator++();
}

std::chrono::utc_clock::duration f20(std::chrono::utc_clock::duration &a0,
                                     int a1) {
  return a0.operator++(a1);
}

std::chrono::utc_clock::duration &f21(std::chrono::utc_clock::duration &a0) {
  return a0.operator--();
}

std::chrono::utc_clock::duration f22(std::chrono::utc_clock::duration &a0,
                                     int a1) {
  return a0.operator--(a1);
}

std::chrono::utc_clock::duration &
f23(std::chrono::utc_clock::duration &a0,
    const std::chrono::utc_clock::duration &a1) {
  return a0.operator+=(a1);
}

std::chrono::utc_clock::duration &
f24(std::chrono::utc_clock::duration &a0,
    const std::chrono::utc_clock::duration &a1) {
  return a0.operator-=(a1);
}

std::chrono::utc_clock::duration &
f25(std::chrono::utc_clock::duration &a0,
    const std::chrono::duration<long long, std::ratio<1, 1000000>>::rep &a1) {
  return a0.operator*=(a1);
}

std::chrono::utc_clock::duration &
f26(std::chrono::utc_clock::duration &a0,
    const std::chrono::duration<long long, std::ratio<1, 1000000>>::rep &a1) {
  return a0.operator/=(a1);
}

std::chrono::utc_clock::duration &
f27(std::chrono::utc_clock::duration &a0,
    const std::chrono::duration<long long, std::ratio<1, 1000000>>::rep &a1) {
  return a0.operator%=(a1);
}

std::chrono::utc_clock::duration &
f28(std::chrono::utc_clock::duration &a0,
    const std::chrono::utc_clock::duration &a1) {
  return a0.operator%=(a1);
}

std::chrono::utc_clock::duration f29() {
  return std::chrono::duration<long long, std::ratio<1, 1000000>>::zero();
}

std::chrono::utc_clock::duration f30() {
  return std::chrono::duration<long long, std::ratio<1, 1000000>>::min();
}

std::chrono::utc_clock::duration f31() {
  return std::chrono::duration<long long, std::ratio<1, 1000000>>::max();
}

template <typename T1> std::chrono::utc_clock::duration f32(const T1 &a0) {
  return std::chrono::utc_clock::duration(a0);
}

template <typename T1, typename T2>
std::chrono::utc_clock::duration f33(const std::chrono::duration<T1, T2> &a0) {
  return std::chrono::utc_clock::duration(a0);
}

using t10 = std::chrono::time_point<std::chrono::utc_clock>::clock;

using t11 = std::chrono::time_point<std::chrono::utc_clock>::duration;

using t12 = std::chrono::time_point<std::chrono::utc_clock>::rep;

using t13 = std::chrono::time_point<std::chrono::utc_clock>::period;

std::chrono::utc_clock::time_point f34() {
  return std::chrono::utc_clock::time_point();
}

std::chrono::utc_clock::time_point
f35(const std::chrono::time_point<std::chrono::utc_clock>::duration &a0) {
  return std::chrono::utc_clock::time_point(a0);
}

std::chrono::time_point<std::chrono::utc_clock>::duration
f36(const std::chrono::utc_clock::time_point &a0) {
  return a0.time_since_epoch();
}

std::chrono::utc_clock::time_point &
f37(std::chrono::utc_clock::time_point &a0) {
  return a0.operator++();
}

std::chrono::utc_clock::time_point f38(std::chrono::utc_clock::time_point &a0,
                                       int a1) {
  return a0.operator++(a1);
}

std::chrono::utc_clock::time_point &
f39(std::chrono::utc_clock::time_point &a0) {
  return a0.operator--();
}

std::chrono::utc_clock::time_point f40(std::chrono::utc_clock::time_point &a0,
                                       int a1) {
  return a0.operator--(a1);
}

std::chrono::utc_clock::time_point &
f41(std::chrono::utc_clock::time_point &a0,
    const std::chrono::time_point<std::chrono::utc_clock>::duration &a1) {
  return a0.operator+=(a1);
}

std::chrono::utc_clock::time_point &
f42(std::chrono::utc_clock::time_point &a0,
    const std::chrono::time_point<std::chrono::utc_clock>::duration &a1) {
  return a0.operator-=(a1);
}

std::chrono::utc_clock::time_point f43() {
  return std::chrono::time_point<std::chrono::utc_clock>::min();
}

std::chrono::utc_clock::time_point f44() {
  return std::chrono::time_point<std::chrono::utc_clock>::max();
}

template <typename T1>
std::chrono::utc_clock::time_point
f45(const std::chrono::time_point<
    std::chrono::time_point<
        std::chrono::utc_clock,
        std::chrono::duration<long long, std::ratio<1, 1000000>>>::clock,
    T1> &a0) {
  return std::chrono::utc_clock::time_point(a0);
}

bool f46() { return std::chrono::utc_clock::is_steady; }

using t14 = std::chrono::utc_seconds;

template <typename T1> using t15 = std::chrono::utc_time<T1>;
