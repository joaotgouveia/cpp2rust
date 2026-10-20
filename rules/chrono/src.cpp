#include "../common/init.h"
#include "../common/traits.h"
#include <chrono>
#include <utility>

template <typename T1, typename T2, typename T3>
T1 f1(const std::chrono::duration<T2, T3> &a0) {
  return std::chrono::duration_cast<T1, T2, T3>(a0);
}

using t1 = std::chrono::high_resolution_clock;

using t2 = std::chrono::hours;

using t3 = std::chrono::microseconds;

using t4 = std::chrono::milliseconds;

using t5 = std::chrono::minutes;

using t6 = std::chrono::nanoseconds;

template <typename T1, typename T2, typename T3>
auto f2(const std::chrono::duration<T1, T2> &a0, const T3 &a1)
    -> decltype(a0 % a1) {
  return a0 % a1;
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::common_type<std::chrono::duration<T1, T2>,
                          std::chrono::duration<T3, T4>>::type
f3(const std::chrono::duration<T1, T2> &a0,
   const std::chrono::duration<T3, T4> &a1) {
  return a0 % a1;
}

template <typename T1, typename T2, typename T3>
auto f4(const std::chrono::duration<T1, T2> &a0, const T3 &a1)
    -> decltype(a0 * a1) {
  return a0 * a1;
}

template <typename T1, typename T2, typename T3>
auto f5(const T1 &a0, const std::chrono::duration<T2, T3> &a1)
    -> decltype(a0 * a1) {
  return a0 * a1;
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::common_type<std::chrono::duration<T1, T2>,
                          std::chrono::duration<T3, T4>>::type
f6(const std::chrono::duration<T1, T2> &a0,
   const std::chrono::duration<T3, T4> &a1) {
  return a0 + a1;
}

template <typename T1, typename T2, typename T3, typename T4>
std::chrono::time_point<
    T3, typename std::common_type<std::chrono::duration<T1, T2>, T4>::type>
f7(const std::chrono::duration<T1, T2> &a0,
   const std::chrono::time_point<T3, T4> &a1) {
  return a0 + a1;
}

template <typename T1, typename T2, typename T3, typename T4>
std::chrono::time_point<
    T1, typename std::common_type<T2, std::chrono::duration<T3, T4>>::type>
f8(const std::chrono::time_point<T1, T2> &a0,
   const std::chrono::duration<T3, T4> &a1) {
  return a0 + a1;
}

template <typename T1, typename T2, typename T3>
typename std::common_type<T2, T3>::type
f9(const std::chrono::time_point<T1, T2> &a0,
   const std::chrono::time_point<T1, T3> &a1) {
  return a0 - a1;
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::common_type<std::chrono::duration<T1, T2>,
                          std::chrono::duration<T3, T4>>::type
f10(const std::chrono::duration<T1, T2> &a0,
    const std::chrono::duration<T3, T4> &a1) {
  return a0 - a1;
}

template <typename T1, typename T2, typename T3, typename T4>
std::chrono::time_point<
    T1, typename std::common_type<T2, std::chrono::duration<T3, T4>>::type>
f11(const std::chrono::time_point<T1, T2> &a0,
    const std::chrono::duration<T3, T4> &a1) {
  return a0 - a1;
}

template <typename T1, typename T2, typename T3>
auto f12(const std::chrono::duration<T1, T2> &a0, const T3 &a1)
    -> decltype(a0 / a1) {
  return a0 / a1;
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::common_type<T1, T3>::type
f13(const std::chrono::duration<T1, T2> &a0,
    const std::chrono::duration<T3, T4> &a1) {
  return a0 / a1;
}

template <typename T1, typename T2, typename T3>
bool f14(const std::chrono::time_point<T1, T2> &a0,
         const std::chrono::time_point<T1, T3> &a1) {
  return a0 < a1;
}

template <typename T1, typename T2, typename T3, typename T4>
bool f15(const std::chrono::duration<T1, T2> &a0,
         const std::chrono::duration<T3, T4> &a1) {
  return a0 < a1;
}

template <typename T1, typename T2, typename T3>
bool f16(const std::chrono::time_point<T1, T2> &a0,
         const std::chrono::time_point<T1, T3> &a1) {
  return a0 <= a1;
}

template <typename T1, typename T2, typename T3, typename T4>
bool f17(const std::chrono::duration<T1, T2> &a0,
         const std::chrono::duration<T3, T4> &a1) {
  return a0 <= a1;
}

template <typename T1, typename T2, typename T3>
bool f18(const std::chrono::time_point<T1, T2> &a0,
         const std::chrono::time_point<T1, T3> &a1) {
  return a0 == a1;
}

template <typename T1, typename T2, typename T3, typename T4>
bool f19(const std::chrono::duration<T1, T2> &a0,
         const std::chrono::duration<T3, T4> &a1) {
  return a0 == a1;
}

template <typename T1, typename T2, typename T3>
bool f20(const std::chrono::time_point<T1, T2> &a0,
         const std::chrono::time_point<T1, T3> &a1) {
  return a0 > a1;
}

template <typename T1, typename T2, typename T3, typename T4>
bool f21(const std::chrono::duration<T1, T2> &a0,
         const std::chrono::duration<T3, T4> &a1) {
  return a0 > a1;
}

template <typename T1, typename T2, typename T3>
bool f22(const std::chrono::time_point<T1, T2> &a0,
         const std::chrono::time_point<T1, T3> &a1) {
  return a0 >= a1;
}

template <typename T1, typename T2, typename T3, typename T4>
bool f23(const std::chrono::duration<T1, T2> &a0,
         const std::chrono::duration<T3, T4> &a1) {
  return a0 >= a1;
}

using t7 = std::chrono::seconds;

using t8 = std::chrono::steady_clock;

using t9 = std::chrono::steady_clock::duration;

using t10 = std::chrono::steady_clock::rep;

using t11 = std::chrono::steady_clock::period;

using t12 = std::chrono::steady_clock::time_point;

std::chrono::steady_clock::time_point f24() {
  return std::chrono::steady_clock::now();
}

std::chrono::steady_clock f25() { return std::chrono::steady_clock(); }

std::chrono::steady_clock f26(const std::chrono::steady_clock &a0) {
  return std::chrono::steady_clock(a0);
}

std::chrono::steady_clock f27(std::chrono::steady_clock &&a0) {
  return std::chrono::steady_clock(std::move(a0));
}

std::chrono::steady_clock &f28(std::chrono::steady_clock &a0,
                               const std::chrono::steady_clock &a1) {
  return a0.operator=(a1);
}

std::chrono::steady_clock &f29(std::chrono::steady_clock &a0,
                               std::chrono::steady_clock &&a1) {
  return a0.operator=(std::move(a1));
}

using t13 = std::chrono::duration<long, std::ratio<1, 1000000000>>::rep;

using t14 = std::chrono::duration<long, std::ratio<1, 1000000000>>::period;

std::chrono::steady_clock::duration f30() {
  return std::chrono::steady_clock::duration();
}

std::chrono::steady_clock::duration
f31(const std::chrono::steady_clock::duration &a0) {
  return std::chrono::steady_clock::duration(a0);
}

std::chrono::steady_clock::duration &
f32(std::chrono::steady_clock::duration &a0,
    const std::chrono::steady_clock::duration &a1) {
  return a0.operator=(a1);
}

std::chrono::duration<long, std::ratio<1, 1000000000>>::rep
f33(const std::chrono::steady_clock::duration &a0) {
  return a0.count();
}

std::chrono::duration<
    typename std::common_type<
        std::chrono::duration<long, std::ratio<1, 1000000000>>::rep>::type,
    std::chrono::duration<long, std::ratio<1, 1000000000>>::period>
f34(const std::chrono::steady_clock::duration &a0) {
  return a0.operator+();
}

std::chrono::duration<
    typename std::common_type<
        std::chrono::duration<long, std::ratio<1, 1000000000>>::rep>::type,
    std::chrono::duration<long, std::ratio<1, 1000000000>>::period>
f35(const std::chrono::steady_clock::duration &a0) {
  return a0.operator-();
}

std::chrono::steady_clock::duration &
f36(std::chrono::steady_clock::duration &a0) {
  return a0.operator++();
}

std::chrono::steady_clock::duration f37(std::chrono::steady_clock::duration &a0,
                                        int a1) {
  return a0.operator++(a1);
}

std::chrono::steady_clock::duration &
f38(std::chrono::steady_clock::duration &a0) {
  return a0.operator--();
}

std::chrono::steady_clock::duration f39(std::chrono::steady_clock::duration &a0,
                                        int a1) {
  return a0.operator--(a1);
}

std::chrono::steady_clock::duration &
f40(std::chrono::steady_clock::duration &a0,
    const std::chrono::steady_clock::duration &a1) {
  return a0.operator+=(a1);
}

std::chrono::steady_clock::duration &
f41(std::chrono::steady_clock::duration &a0,
    const std::chrono::steady_clock::duration &a1) {
  return a0.operator-=(a1);
}

std::chrono::steady_clock::duration &
f42(std::chrono::steady_clock::duration &a0,
    const std::chrono::duration<long, std::ratio<1, 1000000000>>::rep &a1) {
  return a0.operator*=(a1);
}

std::chrono::steady_clock::duration &
f43(std::chrono::steady_clock::duration &a0,
    const std::chrono::duration<long, std::ratio<1, 1000000000>>::rep &a1) {
  return a0.operator/=(a1);
}

std::chrono::steady_clock::duration f44() {
  return std::chrono::duration<long, std::ratio<1, 1000000000>>::zero();
}

std::chrono::steady_clock::duration f45() {
  return std::chrono::duration<long, std::ratio<1, 1000000000>>::min();
}

std::chrono::steady_clock::duration f46() {
  return std::chrono::duration<long, std::ratio<1, 1000000000>>::max();
}

template <typename T1> std::chrono::steady_clock::duration f47(const T1 &a0) {
  return std::chrono::steady_clock::duration(a0);
}

template <typename T1, typename T2>
std::chrono::steady_clock::duration
f48(const std::chrono::duration<T1, T2> &a0) {
  return std::chrono::steady_clock::duration(a0);
}

std::chrono::steady_clock::duration &
f49(std::chrono::steady_clock::duration &a0,
    const std::chrono::duration<long, std::ratio<1, 1000000000>>::rep &a1) {
  return a0.operator%=(a1);
}

std::chrono::steady_clock::duration &
f50(std::chrono::steady_clock::duration &a0,
    const std::chrono::steady_clock::duration &a1) {
  return a0.operator%=(a1);
}

using t15 = std::ratio<1, 1000000000>::type;

using t16 = std::chrono::time_point<
    std::chrono::steady_clock,
    std::chrono::duration<long, std::ratio<1, 1000000000>>>::clock;

using t17 = std::chrono::time_point<
    std::chrono::steady_clock,
    std::chrono::duration<long, std::ratio<1, 1000000000>>>::duration;

using t18 = std::chrono::time_point<
    std::chrono::steady_clock,
    std::chrono::duration<long, std::ratio<1, 1000000000>>>::rep;

using t19 = std::chrono::time_point<
    std::chrono::steady_clock,
    std::chrono::duration<long, std::ratio<1, 1000000000>>>::period;

std::chrono::steady_clock::time_point f51() {
  return std::chrono::steady_clock::time_point();
}

std::chrono::steady_clock::time_point
f52(const std::chrono::time_point<
    std::chrono::steady_clock,
    std::chrono::duration<long, std::ratio<1, 1000000000>>>::duration &a0) {
  return std::chrono::steady_clock::time_point(a0);
}

std::chrono::time_point<
    std::chrono::steady_clock,
    std::chrono::duration<long, std::ratio<1, 1000000000>>>::duration
f53(const std::chrono::steady_clock::time_point &a0) {
  return a0.time_since_epoch();
}

std::chrono::steady_clock::time_point &
f54(std::chrono::steady_clock::time_point &a0,
    const std::chrono::time_point<
        std::chrono::steady_clock,
        std::chrono::duration<long, std::ratio<1, 1000000000>>>::duration &a1) {
  return a0.operator+=(a1);
}

std::chrono::steady_clock::time_point &
f55(std::chrono::steady_clock::time_point &a0,
    const std::chrono::time_point<
        std::chrono::steady_clock,
        std::chrono::duration<long, std::ratio<1, 1000000000>>>::duration &a1) {
  return a0.operator-=(a1);
}

std::chrono::steady_clock::time_point f56() {
  return std::chrono::time_point<
      std::chrono::steady_clock,
      std::chrono::duration<long, std::ratio<1, 1000000000>>>::min();
}

std::chrono::steady_clock::time_point f57() {
  return std::chrono::time_point<
      std::chrono::steady_clock,
      std::chrono::duration<long, std::ratio<1, 1000000000>>>::max();
}

template <typename T1>
std::chrono::steady_clock::time_point
f58(const std::chrono::time_point<
    std::chrono::time_point<
        std::chrono::steady_clock,
        std::chrono::duration<long, std::ratio<1, 1000000000>>>::clock,
    T1> &a0) {
  return std::chrono::steady_clock::time_point(a0);
}

bool f59() { return std::chrono::steady_clock::is_steady; }

using t20 = std::chrono::system_clock;

using t21 = std::chrono::system_clock::duration;

using t22 = std::chrono::system_clock::rep;

using t23 = std::chrono::system_clock::period;

using t24 = std::chrono::system_clock::time_point;

std::chrono::system_clock::time_point f60() {
  return std::chrono::system_clock::now();
}

std::time_t f61(const std::chrono::system_clock::time_point &a0) {
  return std::chrono::system_clock::to_time_t(a0);
}

std::chrono::system_clock::time_point f62(std::time_t a0) {
  return std::chrono::system_clock::from_time_t(a0);
}

std::chrono::system_clock f63() { return std::chrono::system_clock(); }

std::chrono::system_clock f64(const std::chrono::system_clock &a0) {
  return std::chrono::system_clock(a0);
}

std::chrono::system_clock f65(std::chrono::system_clock &&a0) {
  return std::chrono::system_clock(std::move(a0));
}

std::chrono::system_clock &f66(std::chrono::system_clock &a0,
                               const std::chrono::system_clock &a1) {
  return a0.operator=(a1);
}

std::chrono::system_clock &f67(std::chrono::system_clock &a0,
                               std::chrono::system_clock &&a1) {
  return a0.operator=(std::move(a1));
}

std::chrono::system_clock::duration f68() {
  return std::chrono::system_clock::duration();
}

std::chrono::system_clock::duration
f69(const std::chrono::system_clock::duration &a0) {
  return std::chrono::system_clock::duration(a0);
}

std::chrono::system_clock::duration &
f70(std::chrono::system_clock::duration &a0,
    const std::chrono::system_clock::duration &a1) {
  return a0.operator=(a1);
}

std::chrono::duration<long, std::ratio<1, 1000000000>>::rep
f71(const std::chrono::system_clock::duration &a0) {
  return a0.count();
}

std::chrono::duration<
    typename std::common_type<
        std::chrono::duration<long, std::ratio<1, 1000000000>>::rep>::type,
    std::chrono::duration<long, std::ratio<1, 1000000000>>::period>
f72(const std::chrono::system_clock::duration &a0) {
  return a0.operator+();
}

std::chrono::duration<
    typename std::common_type<
        std::chrono::duration<long, std::ratio<1, 1000000000>>::rep>::type,
    std::chrono::duration<long, std::ratio<1, 1000000000>>::period>
f73(const std::chrono::system_clock::duration &a0) {
  return a0.operator-();
}

std::chrono::system_clock::duration &
f74(std::chrono::system_clock::duration &a0) {
  return a0.operator++();
}

std::chrono::system_clock::duration f75(std::chrono::system_clock::duration &a0,
                                        int a1) {
  return a0.operator++(a1);
}

std::chrono::system_clock::duration &
f76(std::chrono::system_clock::duration &a0) {
  return a0.operator--();
}

std::chrono::system_clock::duration f77(std::chrono::system_clock::duration &a0,
                                        int a1) {
  return a0.operator--(a1);
}

std::chrono::system_clock::duration &
f78(std::chrono::system_clock::duration &a0,
    const std::chrono::system_clock::duration &a1) {
  return a0.operator+=(a1);
}

std::chrono::system_clock::duration &
f79(std::chrono::system_clock::duration &a0,
    const std::chrono::system_clock::duration &a1) {
  return a0.operator-=(a1);
}

std::chrono::system_clock::duration &
f80(std::chrono::system_clock::duration &a0,
    const std::chrono::duration<long, std::ratio<1, 1000000000>>::rep &a1) {
  return a0.operator*=(a1);
}

std::chrono::system_clock::duration &
f81(std::chrono::system_clock::duration &a0,
    const std::chrono::duration<long, std::ratio<1, 1000000000>>::rep &a1) {
  return a0.operator/=(a1);
}

std::chrono::system_clock::duration f82() {
  return std::chrono::duration<long, std::ratio<1, 1000000000>>::zero();
}

std::chrono::system_clock::duration f83() {
  return std::chrono::duration<long, std::ratio<1, 1000000000>>::min();
}

std::chrono::system_clock::duration f84() {
  return std::chrono::duration<long, std::ratio<1, 1000000000>>::max();
}

template <typename T1> std::chrono::system_clock::duration f85(const T1 &a0) {
  return std::chrono::system_clock::duration(a0);
}

template <typename T1, typename T2>
std::chrono::system_clock::duration
f86(const std::chrono::duration<T1, T2> &a0) {
  return std::chrono::system_clock::duration(a0);
}

std::chrono::system_clock::duration &
f87(std::chrono::system_clock::duration &a0,
    const std::chrono::duration<long, std::ratio<1, 1000000000>>::rep &a1) {
  return a0.operator%=(a1);
}

std::chrono::system_clock::duration &
f88(std::chrono::system_clock::duration &a0,
    const std::chrono::system_clock::duration &a1) {
  return a0.operator%=(a1);
}

using t25 = std::chrono::time_point<
    std::chrono::system_clock,
    std::chrono::duration<long, std::ratio<1, 1000000000>>>::clock;

using t26 = std::chrono::time_point<
    std::chrono::system_clock,
    std::chrono::duration<long, std::ratio<1, 1000000000>>>::duration;

using t27 = std::chrono::time_point<
    std::chrono::system_clock,
    std::chrono::duration<long, std::ratio<1, 1000000000>>>::rep;

using t28 = std::chrono::time_point<
    std::chrono::system_clock,
    std::chrono::duration<long, std::ratio<1, 1000000000>>>::period;

std::chrono::system_clock::time_point f89() {
  return std::chrono::system_clock::time_point();
}

std::chrono::system_clock::time_point
f90(const std::chrono::time_point<
    std::chrono::system_clock,
    std::chrono::duration<long, std::ratio<1, 1000000000>>>::duration &a0) {
  return std::chrono::system_clock::time_point(a0);
}

std::chrono::time_point<
    std::chrono::system_clock,
    std::chrono::duration<long, std::ratio<1, 1000000000>>>::duration
f91(const std::chrono::system_clock::time_point &a0) {
  return a0.time_since_epoch();
}

std::chrono::system_clock::time_point &
f92(std::chrono::system_clock::time_point &a0,
    const std::chrono::time_point<
        std::chrono::system_clock,
        std::chrono::duration<long, std::ratio<1, 1000000000>>>::duration &a1) {
  return a0.operator+=(a1);
}

std::chrono::system_clock::time_point &
f93(std::chrono::system_clock::time_point &a0,
    const std::chrono::time_point<
        std::chrono::system_clock,
        std::chrono::duration<long, std::ratio<1, 1000000000>>>::duration &a1) {
  return a0.operator-=(a1);
}

std::chrono::system_clock::time_point f94() {
  return std::chrono::time_point<
      std::chrono::system_clock,
      std::chrono::duration<long, std::ratio<1, 1000000000>>>::min();
}

std::chrono::system_clock::time_point f95() {
  return std::chrono::time_point<
      std::chrono::system_clock,
      std::chrono::duration<long, std::ratio<1, 1000000000>>>::max();
}

template <typename T1>
std::chrono::system_clock::time_point
f96(const std::chrono::time_point<
    std::chrono::time_point<
        std::chrono::system_clock,
        std::chrono::duration<long, std::ratio<1, 1000000000>>>::clock,
    T1> &a0) {
  return std::chrono::system_clock::time_point(a0);
}

bool f97() { return std::chrono::system_clock::is_steady; }

template <typename T1, typename T2, typename T3>
std::chrono::time_point<T2, T1> f98(const std::chrono::time_point<T2, T3> &a0) {
  return std::chrono::time_point_cast<T1, T2, T3>(a0);
}
