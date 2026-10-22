#include "../common/init.h"
#include "../common/traits.h"
#include <chrono>
#include <ratio>
#include <utility>

using t1 = std::chrono::system_clock;

using t2 = std::chrono::system_clock::duration;

using t3 = std::chrono::system_clock::rep;

using t4 = std::chrono::system_clock::period;

using t5 = std::chrono::system_clock::time_point;

std::chrono::system_clock::time_point f1() {
  return std::chrono::system_clock::now();
}

std::time_t f2(const std::chrono::system_clock::time_point &a0) {
  return std::chrono::system_clock::to_time_t(a0);
}

std::chrono::system_clock::time_point f3(std::time_t a0) {
  return std::chrono::system_clock::from_time_t(a0);
}

std::chrono::system_clock f4() { return std::chrono::system_clock(); }

std::chrono::system_clock f5(const std::chrono::system_clock &a0) {
  return std::chrono::system_clock(a0);
}

std::chrono::system_clock f6(std::chrono::system_clock &&a0) {
  return std::chrono::system_clock(std::move(a0));
}

std::chrono::system_clock &f7(std::chrono::system_clock &a0,
                              const std::chrono::system_clock &a1) {
  return a0.operator=(a1);
}

std::chrono::system_clock &f8(std::chrono::system_clock &a0,
                              std::chrono::system_clock &&a1) {
  return a0.operator=(std::move(a1));
}

using t6 = std::chrono::duration<long long, std::ratio<1, 1000000>>::rep;

using t7 = std::chrono::duration<long long, std::ratio<1, 1000000>>::period;

std::chrono::system_clock::duration f9() {
  return std::chrono::system_clock::duration();
}

std::chrono::duration<long long, std::ratio<1, 1000000>>::rep
f10(const std::chrono::system_clock::duration &a0) {
  return a0.count();
}

typename std::common_type<std::chrono::system_clock::duration>::type
f11(const std::chrono::system_clock::duration &a0) {
  return a0.operator+();
}

typename std::common_type<std::chrono::system_clock::duration>::type
f12(const std::chrono::system_clock::duration &a0) {
  return a0.operator-();
}

std::chrono::system_clock::duration &
f13(std::chrono::system_clock::duration &a0) {
  return a0.operator++();
}

std::chrono::system_clock::duration f14(std::chrono::system_clock::duration &a0,
                                        int a1) {
  return a0.operator++(a1);
}

std::chrono::system_clock::duration &
f15(std::chrono::system_clock::duration &a0) {
  return a0.operator--();
}

std::chrono::system_clock::duration f16(std::chrono::system_clock::duration &a0,
                                        int a1) {
  return a0.operator--(a1);
}

std::chrono::system_clock::duration &
f17(std::chrono::system_clock::duration &a0,
    const std::chrono::system_clock::duration &a1) {
  return a0.operator+=(a1);
}

std::chrono::system_clock::duration &
f18(std::chrono::system_clock::duration &a0,
    const std::chrono::system_clock::duration &a1) {
  return a0.operator-=(a1);
}

std::chrono::system_clock::duration &
f19(std::chrono::system_clock::duration &a0,
    const std::chrono::duration<long long, std::ratio<1, 1000000>>::rep &a1) {
  return a0.operator*=(a1);
}

std::chrono::system_clock::duration &
f20(std::chrono::system_clock::duration &a0,
    const std::chrono::duration<long long, std::ratio<1, 1000000>>::rep &a1) {
  return a0.operator/=(a1);
}

std::chrono::system_clock::duration &
f21(std::chrono::system_clock::duration &a0,
    const std::chrono::duration<long long, std::ratio<1, 1000000>>::rep &a1) {
  return a0.operator%=(a1);
}

std::chrono::system_clock::duration &
f22(std::chrono::system_clock::duration &a0,
    const std::chrono::system_clock::duration &a1) {
  return a0.operator%=(a1);
}

std::chrono::system_clock::duration f23() {
  return std::chrono::duration<long long, std::ratio<1, 1000000>>::zero();
}

std::chrono::system_clock::duration f24() {
  return std::chrono::duration<long long, std::ratio<1, 1000000>>::min();
}

std::chrono::system_clock::duration f25() {
  return std::chrono::duration<long long, std::ratio<1, 1000000>>::max();
}

template <typename T1,
          typename std::enable_if<
              std::is_convertible<const T1 &, long long>::value &&
                  (std::integral_constant<bool, false>::value ||
                   !std::chrono::treat_as_floating_point<T1>::value),
              int>::type T2 = 0>
std::chrono::system_clock::duration f26(const T1 &a0) {
  return std::chrono::system_clock::duration(a0);
}

using t8 = std::ratio<1, 1000000>::type;

using t9 = std::chrono::time_point<std::chrono::system_clock>::clock;

using t10 = std::chrono::time_point<std::chrono::system_clock>::duration;

using t11 = std::chrono::time_point<std::chrono::system_clock>::rep;

using t12 = std::chrono::time_point<std::chrono::system_clock>::period;

std::chrono::system_clock::time_point f27() {
  return std::chrono::system_clock::time_point();
}

std::chrono::system_clock::time_point
f28(const std::chrono::time_point<std::chrono::system_clock>::duration &a0) {
  return std::chrono::system_clock::time_point(a0);
}

std::chrono::time_point<std::chrono::system_clock>::duration
f29(const std::chrono::system_clock::time_point &a0) {
  return a0.time_since_epoch();
}

std::chrono::system_clock::time_point &
f30(std::chrono::system_clock::time_point &a0,
    const std::chrono::time_point<std::chrono::system_clock>::duration &a1) {
  return a0.operator+=(a1);
}

std::chrono::system_clock::time_point &
f31(std::chrono::system_clock::time_point &a0,
    const std::chrono::time_point<std::chrono::system_clock>::duration &a1) {
  return a0.operator-=(a1);
}

std::chrono::system_clock::time_point f32() {
  return std::chrono::time_point<std::chrono::system_clock>::min();
}

std::chrono::system_clock::time_point f33() {
  return std::chrono::time_point<std::chrono::system_clock>::max();
}

template <typename T1,
          typename std::enable_if<
              std::is_convertible<
                  T1, typename std::chrono::system_clock::duration>::value,
              int>::type T2 = 0>
std::chrono::system_clock::time_point
f34(const std::chrono::time_point<
    std::chrono::time_point<
        std::chrono::system_clock,
        std::chrono::duration<long long, std::ratio<1, 1000000>>>::clock,
    T1> &a0) {
  return std::chrono::system_clock::time_point(a0);
}

bool f35() { return std::chrono::system_clock::is_steady; }
