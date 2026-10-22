#include "../common/init.h"
#include "../common/traits.h"
#include <chrono>
#include <ratio>
#include <type_traits>
#include <utility>

using t1 = std::chrono::tai_clock;

using t2 = std::chrono::tai_clock::rep;

using t3 = std::chrono::tai_clock::period;

using t4 = std::chrono::tai_clock::duration;

using t5 = std::chrono::tai_clock::time_point;

std::chrono::tai_clock::time_point f1() {
  return std::chrono::tai_clock::now();
}

std::chrono::tai_clock f2() { return std::chrono::tai_clock(); }

std::chrono::tai_clock f3(const std::chrono::tai_clock &a0) {
  return std::chrono::tai_clock(a0);
}

std::chrono::tai_clock f4(std::chrono::tai_clock &&a0) {
  return std::chrono::tai_clock(std::move(a0));
}

std::chrono::tai_clock &f5(std::chrono::tai_clock &a0,
                           const std::chrono::tai_clock &a1) {
  return a0.operator=(a1);
}

std::chrono::tai_clock &f6(std::chrono::tai_clock &a0,
                           std::chrono::tai_clock &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
std::chrono::utc_time<std::common_type_t<T1, std::chrono::seconds>>
f7(const std::chrono::tai_time<T1> &a0) {
  return std::chrono::tai_clock::to_utc(a0);
}

template <typename T1>
std::chrono::tai_time<std::common_type_t<T1, std::chrono::seconds>>
f8(const std::chrono::utc_time<T1> &a0) {
  return std::chrono::tai_clock::from_utc(a0);
}

using t6 = std::ratio<1, 1000000>::type;

using t7 = std::chrono::duration<long long, std::ratio<1, 1000000>>::rep;

using t8 = std::chrono::duration<long long, std::ratio<1, 1000000>>::period;

std::chrono::tai_clock::duration f9() {
  return std::chrono::tai_clock::duration();
}

std::chrono::duration<long long, std::ratio<1, 1000000>>::rep
f10(const std::chrono::tai_clock::duration &a0) {
  return a0.count();
}

typename std::common_type<std::chrono::tai_clock::duration>::type
f11(const std::chrono::tai_clock::duration &a0) {
  return a0.operator+();
}

typename std::common_type<std::chrono::tai_clock::duration>::type
f12(const std::chrono::tai_clock::duration &a0) {
  return a0.operator-();
}

std::chrono::tai_clock::duration &f13(std::chrono::tai_clock::duration &a0) {
  return a0.operator++();
}

std::chrono::tai_clock::duration f14(std::chrono::tai_clock::duration &a0,
                                     int a1) {
  return a0.operator++(a1);
}

std::chrono::tai_clock::duration &f15(std::chrono::tai_clock::duration &a0) {
  return a0.operator--();
}

std::chrono::tai_clock::duration f16(std::chrono::tai_clock::duration &a0,
                                     int a1) {
  return a0.operator--(a1);
}

std::chrono::tai_clock::duration &
f17(std::chrono::tai_clock::duration &a0,
    const std::chrono::tai_clock::duration &a1) {
  return a0.operator+=(a1);
}

std::chrono::tai_clock::duration &
f18(std::chrono::tai_clock::duration &a0,
    const std::chrono::tai_clock::duration &a1) {
  return a0.operator-=(a1);
}

std::chrono::tai_clock::duration &
f19(std::chrono::tai_clock::duration &a0,
    const std::chrono::duration<long long, std::ratio<1, 1000000>>::rep &a1) {
  return a0.operator*=(a1);
}

std::chrono::tai_clock::duration &
f20(std::chrono::tai_clock::duration &a0,
    const std::chrono::duration<long long, std::ratio<1, 1000000>>::rep &a1) {
  return a0.operator/=(a1);
}

std::chrono::tai_clock::duration &
f21(std::chrono::tai_clock::duration &a0,
    const std::chrono::duration<long long, std::ratio<1, 1000000>>::rep &a1) {
  return a0.operator%=(a1);
}

std::chrono::tai_clock::duration &
f22(std::chrono::tai_clock::duration &a0,
    const std::chrono::tai_clock::duration &a1) {
  return a0.operator%=(a1);
}

std::chrono::tai_clock::duration f23() {
  return std::chrono::duration<long long, std::ratio<1, 1000000>>::zero();
}

std::chrono::tai_clock::duration f24() {
  return std::chrono::duration<long long, std::ratio<1, 1000000>>::min();
}

std::chrono::tai_clock::duration f25() {
  return std::chrono::duration<long long, std::ratio<1, 1000000>>::max();
}

template <typename T1> std::chrono::tai_clock::duration f26(const T1 &a0) {
  return std::chrono::tai_clock::duration(a0);
}

template <typename T1, typename T2>
std::chrono::tai_clock::duration f27(const std::chrono::duration<T1, T2> &a0) {
  return std::chrono::tai_clock::duration(a0);
}

using t9 = std::chrono::time_point<std::chrono::tai_clock>::clock;

using t10 = std::chrono::time_point<std::chrono::tai_clock>::duration;

using t11 = std::chrono::time_point<std::chrono::tai_clock>::rep;

using t12 = std::chrono::time_point<std::chrono::tai_clock>::period;

std::chrono::tai_clock::time_point f28() {
  return std::chrono::tai_clock::time_point();
}

std::chrono::tai_clock::time_point
f29(const std::chrono::time_point<std::chrono::tai_clock>::duration &a0) {
  return std::chrono::tai_clock::time_point(a0);
}

std::chrono::time_point<std::chrono::tai_clock>::duration
f30(const std::chrono::tai_clock::time_point &a0) {
  return a0.time_since_epoch();
}

std::chrono::tai_clock::time_point &
f31(std::chrono::tai_clock::time_point &a0) {
  return a0.operator++();
}

std::chrono::tai_clock::time_point f32(std::chrono::tai_clock::time_point &a0,
                                       int a1) {
  return a0.operator++(a1);
}

std::chrono::tai_clock::time_point &
f33(std::chrono::tai_clock::time_point &a0) {
  return a0.operator--();
}

std::chrono::tai_clock::time_point f34(std::chrono::tai_clock::time_point &a0,
                                       int a1) {
  return a0.operator--(a1);
}

std::chrono::tai_clock::time_point &
f35(std::chrono::tai_clock::time_point &a0,
    const std::chrono::time_point<std::chrono::tai_clock>::duration &a1) {
  return a0.operator+=(a1);
}

std::chrono::tai_clock::time_point &
f36(std::chrono::tai_clock::time_point &a0,
    const std::chrono::time_point<std::chrono::tai_clock>::duration &a1) {
  return a0.operator-=(a1);
}

std::chrono::tai_clock::time_point f37() {
  return std::chrono::time_point<std::chrono::tai_clock>::min();
}

std::chrono::tai_clock::time_point f38() {
  return std::chrono::time_point<std::chrono::tai_clock>::max();
}

template <typename T1>
std::chrono::tai_clock::time_point
f39(const std::chrono::time_point<
    std::chrono::time_point<
        std::chrono::tai_clock,
        std::chrono::duration<long long, std::ratio<1, 1000000>>>::clock,
    T1> &a0) {
  return std::chrono::tai_clock::time_point(a0);
}

bool f40() { return std::chrono::tai_clock::is_steady; }

using t13 = std::chrono::tai_seconds;

template <typename T1> using t14 = std::chrono::tai_time<T1>;
