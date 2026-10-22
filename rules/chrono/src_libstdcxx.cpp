#include "../common/init.h"
#include "../common/traits.h"
#include <chrono>
#include <utility>

using t1 = std::chrono::high_resolution_clock;

using t2 = std::chrono::hours;

using t3 = std::chrono::microseconds;

using t4 = std::chrono::milliseconds;

using t5 = std::chrono::minutes;

using t6 = std::chrono::nanoseconds;

template <typename T1, typename T2, typename T3, typename T4>
typename std::common_type<std::chrono::duration<T1, T2>,
                          std::chrono::duration<T3, T4>>::type
f1(const std::chrono::duration<T1, T2> &a0,
   const std::chrono::duration<T3, T4> &a1) {
  return a0 % a1;
}

template <typename T1, typename T2, typename T3>
std::chrono::duration<
    typename std::enable_if<
        std::is_convertible<T3 const &,
                            typename std::common_type<T1, T3>::type>::value,
        typename std::common_type<T1, T3>::type>::type,
    T2>
f2(const std::chrono::duration<T1, T2> &a0, const T3 &a1) {
  return a0 * a1;
}

template <typename T1, typename T2, typename T3>
std::chrono::duration<
    typename std::enable_if<
        std::is_convertible<T1 const &,
                            typename std::common_type<T2, T1>::type>::value,
        typename std::common_type<T2, T1>::type>::type,
    T3>
f3(const T1 &a0, const std::chrono::duration<T2, T3> &a1) {
  return a0 * a1;
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::common_type<std::chrono::duration<T1, T2>,
                          std::chrono::duration<T3, T4>>::type
f4(const std::chrono::duration<T1, T2> &a0,
   const std::chrono::duration<T3, T4> &a1) {
  return a0 + a1;
}

template <typename T1, typename T2, typename T3, typename T4>
std::chrono::time_point<
    T3, typename std::common_type<std::chrono::duration<T1, T2>, T4>::type>
f5(const std::chrono::duration<T1, T2> &a0,
   const std::chrono::time_point<T3, T4> &a1) {
  return a0 + a1;
}

template <typename T1, typename T2, typename T3, typename T4>
std::chrono::time_point<
    T1, typename std::common_type<T2, std::chrono::duration<T3, T4>>::type>
f6(const std::chrono::time_point<T1, T2> &a0,
   const std::chrono::duration<T3, T4> &a1) {
  return a0 + a1;
}

template <typename T1, typename T2, typename T3>
typename std::common_type<T2, T3>::type
f7(const std::chrono::time_point<T1, T2> &a0,
   const std::chrono::time_point<T1, T3> &a1) {
  return a0 - a1;
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::common_type<std::chrono::duration<T1, T2>,
                          std::chrono::duration<T3, T4>>::type
f8(const std::chrono::duration<T1, T2> &a0,
   const std::chrono::duration<T3, T4> &a1) {
  return a0 - a1;
}

template <typename T1, typename T2, typename T3, typename T4>
std::chrono::time_point<
    T1, typename std::common_type<T2, std::chrono::duration<T3, T4>>::type>
f9(const std::chrono::time_point<T1, T2> &a0,
   const std::chrono::duration<T3, T4> &a1) {
  return a0 - a1;
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::common_type<T1, T3>::type
f10(const std::chrono::duration<T1, T2> &a0,
    const std::chrono::duration<T3, T4> &a1) {
  return a0 / a1;
}

template <typename T1, typename T2, typename T3>
bool f11(const std::chrono::time_point<T1, T2> &a0,
         const std::chrono::time_point<T1, T3> &a1) {
  return a0 < a1;
}

template <typename T1, typename T2, typename T3, typename T4>
bool f12(const std::chrono::duration<T1, T2> &a0,
         const std::chrono::duration<T3, T4> &a1) {
  return a0 < a1;
}

template <typename T1, typename T2, typename T3>
bool f13(const std::chrono::time_point<T1, T2> &a0,
         const std::chrono::time_point<T1, T3> &a1) {
  return a0 <= a1;
}

template <typename T1, typename T2, typename T3, typename T4>
bool f14(const std::chrono::duration<T1, T2> &a0,
         const std::chrono::duration<T3, T4> &a1) {
  return a0 <= a1;
}

template <typename T1, typename T2, typename T3>
bool f15(const std::chrono::time_point<T1, T2> &a0,
         const std::chrono::time_point<T1, T3> &a1) {
  return a0 == a1;
}

template <typename T1, typename T2, typename T3, typename T4>
bool f16(const std::chrono::duration<T1, T2> &a0,
         const std::chrono::duration<T3, T4> &a1) {
  return a0 == a1;
}

template <typename T1, typename T2, typename T3>
bool f17(const std::chrono::time_point<T1, T2> &a0,
         const std::chrono::time_point<T1, T3> &a1) {
  return a0 > a1;
}

template <typename T1, typename T2, typename T3, typename T4>
bool f18(const std::chrono::duration<T1, T2> &a0,
         const std::chrono::duration<T3, T4> &a1) {
  return a0 > a1;
}

template <typename T1, typename T2, typename T3>
bool f19(const std::chrono::time_point<T1, T2> &a0,
         const std::chrono::time_point<T1, T3> &a1) {
  return a0 >= a1;
}

template <typename T1, typename T2, typename T3, typename T4>
bool f20(const std::chrono::duration<T1, T2> &a0,
         const std::chrono::duration<T3, T4> &a1) {
  return a0 >= a1;
}

using t7 = std::chrono::seconds;

using t8 = std::chrono::steady_clock;

using t9 = std::chrono::steady_clock::duration;

using t10 = std::chrono::steady_clock::rep;

using t11 = std::chrono::steady_clock::period;

using t12 = std::chrono::steady_clock::time_point;

std::chrono::steady_clock::time_point f21() {
  return std::chrono::steady_clock::now();
}

std::chrono::steady_clock f22() { return std::chrono::steady_clock(); }

std::chrono::steady_clock f23(const std::chrono::steady_clock &a0) {
  return std::chrono::steady_clock(a0);
}

std::chrono::steady_clock f24(std::chrono::steady_clock &&a0) {
  return std::chrono::steady_clock(std::move(a0));
}

std::chrono::steady_clock &f25(std::chrono::steady_clock &a0,
                               const std::chrono::steady_clock &a1) {
  return a0.operator=(a1);
}

std::chrono::steady_clock &f26(std::chrono::steady_clock &a0,
                               std::chrono::steady_clock &&a1) {
  return a0.operator=(std::move(a1));
}

using t13 = std::chrono::duration<long, std::ratio<1, 1000000000>>::rep;

using t14 = std::chrono::duration<long, std::ratio<1, 1000000000>>::period;

std::chrono::steady_clock::duration f27() {
  return std::chrono::steady_clock::duration();
}

std::chrono::steady_clock::duration
f28(const std::chrono::steady_clock::duration &a0) {
  return std::chrono::steady_clock::duration(a0);
}

std::chrono::steady_clock::duration &
f29(std::chrono::steady_clock::duration &a0,
    const std::chrono::steady_clock::duration &a1) {
  return a0.operator=(a1);
}

std::chrono::duration<long, std::ratio<1, 1000000000>>::rep
f30(const std::chrono::steady_clock::duration &a0) {
  return a0.count();
}

std::chrono::duration<
    typename std::common_type<
        std::chrono::duration<long, std::ratio<1, 1000000000>>::rep>::type,
    std::chrono::duration<long, std::ratio<1, 1000000000>>::period>
f31(const std::chrono::steady_clock::duration &a0) {
  return a0.operator+();
}

std::chrono::duration<
    typename std::common_type<
        std::chrono::duration<long, std::ratio<1, 1000000000>>::rep>::type,
    std::chrono::duration<long, std::ratio<1, 1000000000>>::period>
f32(const std::chrono::steady_clock::duration &a0) {
  return a0.operator-();
}

std::chrono::steady_clock::duration &
f33(std::chrono::steady_clock::duration &a0) {
  return a0.operator++();
}

std::chrono::steady_clock::duration f34(std::chrono::steady_clock::duration &a0,
                                        int a1) {
  return a0.operator++(a1);
}

std::chrono::steady_clock::duration &
f35(std::chrono::steady_clock::duration &a0) {
  return a0.operator--();
}

std::chrono::steady_clock::duration f36(std::chrono::steady_clock::duration &a0,
                                        int a1) {
  return a0.operator--(a1);
}

std::chrono::steady_clock::duration &
f37(std::chrono::steady_clock::duration &a0,
    const std::chrono::steady_clock::duration &a1) {
  return a0.operator+=(a1);
}

std::chrono::steady_clock::duration &
f38(std::chrono::steady_clock::duration &a0,
    const std::chrono::steady_clock::duration &a1) {
  return a0.operator-=(a1);
}

std::chrono::steady_clock::duration &
f39(std::chrono::steady_clock::duration &a0,
    const std::chrono::duration<long, std::ratio<1, 1000000000>>::rep &a1) {
  return a0.operator*=(a1);
}

std::chrono::steady_clock::duration &
f40(std::chrono::steady_clock::duration &a0,
    const std::chrono::duration<long, std::ratio<1, 1000000000>>::rep &a1) {
  return a0.operator/=(a1);
}

std::chrono::steady_clock::duration f41() {
  return std::chrono::duration<long, std::ratio<1, 1000000000>>::zero();
}

std::chrono::steady_clock::duration f42() {
  return std::chrono::duration<long, std::ratio<1, 1000000000>>::min();
}

std::chrono::steady_clock::duration f43() {
  return std::chrono::duration<long, std::ratio<1, 1000000000>>::max();
}

template <
    typename T1,
    typename T2 = typename std::enable_if<
        std::integral_constant<
            bool,
            std::is_convertible<const T1 &, std::int64_t>::value &&
                std::integral_constant<
                    bool, std::integral_constant<bool, false>::value ||
                              std::integral_constant<
                                  bool, !std::chrono::treat_as_floating_point<
                                            T1>::value>::value>::value>::value,
        void>::type>
std::chrono::steady_clock::duration f44(const T1 &a0) {
  return std::chrono::steady_clock::duration(a0);
}

template <
    typename T1 = std::chrono::duration<long, std::ratio<1, 1000000000>>::rep>
std::chrono::duration<long, std::ratio<1, 1000000000>> &
f45(std::chrono::steady_clock::duration &a0,
    const std::chrono::duration<long, std::ratio<1, 1000000000>>::rep &a1) {
  return a0.operator%=(a1);
}

template <
    typename T1 = std::chrono::duration<long, std::ratio<1, 1000000000>>::rep>
std::chrono::duration<long, std::ratio<1, 1000000000>> &
f46(std::chrono::steady_clock::duration &a0,
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

std::chrono::steady_clock::time_point f47() {
  return std::chrono::steady_clock::time_point();
}

std::chrono::steady_clock::time_point
f48(const std::chrono::time_point<
    std::chrono::steady_clock,
    std::chrono::duration<long, std::ratio<1, 1000000000>>>::duration &a0) {
  return std::chrono::steady_clock::time_point(a0);
}

std::chrono::time_point<
    std::chrono::steady_clock,
    std::chrono::duration<long, std::ratio<1, 1000000000>>>::duration
f49(const std::chrono::steady_clock::time_point &a0) {
  return a0.time_since_epoch();
}

std::chrono::steady_clock::time_point &
f50(std::chrono::steady_clock::time_point &a0,
    const std::chrono::time_point<
        std::chrono::steady_clock,
        std::chrono::duration<long, std::ratio<1, 1000000000>>>::duration &a1) {
  return a0.operator+=(a1);
}

std::chrono::steady_clock::time_point &
f51(std::chrono::steady_clock::time_point &a0,
    const std::chrono::time_point<
        std::chrono::steady_clock,
        std::chrono::duration<long, std::ratio<1, 1000000000>>>::duration &a1) {
  return a0.operator-=(a1);
}

std::chrono::steady_clock::time_point f52() {
  return std::chrono::time_point<
      std::chrono::steady_clock,
      std::chrono::duration<long, std::ratio<1, 1000000000>>>::min();
}

std::chrono::steady_clock::time_point f53() {
  return std::chrono::time_point<
      std::chrono::steady_clock,
      std::chrono::duration<long, std::ratio<1, 1000000000>>>::max();
}

template <
    typename T1,
    typename T2 = typename std::enable_if<
        std::integral_constant<
            bool, std::is_convertible<
                      T1, std::chrono::steady_clock::duration>::value>::value,
        void>::type>
std::chrono::steady_clock::time_point
f54(const std::chrono::time_point<
    std::chrono::time_point<
        std::chrono::steady_clock,
        std::chrono::duration<long, std::ratio<1, 1000000000>>>::clock,
    T1> &a0) {
  return std::chrono::steady_clock::time_point(a0);
}

bool f55() { return std::chrono::steady_clock::is_steady; }

using t20 = std::chrono::system_clock;

using t21 = std::chrono::system_clock::duration;

using t22 = std::chrono::system_clock::rep;

using t23 = std::chrono::system_clock::period;

using t24 = std::chrono::system_clock::time_point;

std::chrono::system_clock::time_point f56() {
  return std::chrono::system_clock::now();
}

std::time_t f57(const std::chrono::system_clock::time_point &a0) {
  return std::chrono::system_clock::to_time_t(a0);
}

std::chrono::system_clock::time_point f58(std::time_t a0) {
  return std::chrono::system_clock::from_time_t(a0);
}

std::chrono::system_clock f59() { return std::chrono::system_clock(); }

std::chrono::system_clock f60(const std::chrono::system_clock &a0) {
  return std::chrono::system_clock(a0);
}

std::chrono::system_clock f61(std::chrono::system_clock &&a0) {
  return std::chrono::system_clock(std::move(a0));
}

std::chrono::system_clock &f62(std::chrono::system_clock &a0,
                               const std::chrono::system_clock &a1) {
  return a0.operator=(a1);
}

std::chrono::system_clock &f63(std::chrono::system_clock &a0,
                               std::chrono::system_clock &&a1) {
  return a0.operator=(std::move(a1));
}

std::chrono::system_clock::duration f64() {
  return std::chrono::system_clock::duration();
}

std::chrono::system_clock::duration
f65(const std::chrono::system_clock::duration &a0) {
  return std::chrono::system_clock::duration(a0);
}

std::chrono::system_clock::duration &
f66(std::chrono::system_clock::duration &a0,
    const std::chrono::system_clock::duration &a1) {
  return a0.operator=(a1);
}

std::chrono::duration<long, std::ratio<1, 1000000000>>::rep
f67(const std::chrono::system_clock::duration &a0) {
  return a0.count();
}

std::chrono::duration<
    typename std::common_type<
        std::chrono::duration<long, std::ratio<1, 1000000000>>::rep>::type,
    std::chrono::duration<long, std::ratio<1, 1000000000>>::period>
f68(const std::chrono::system_clock::duration &a0) {
  return a0.operator+();
}

std::chrono::duration<
    typename std::common_type<
        std::chrono::duration<long, std::ratio<1, 1000000000>>::rep>::type,
    std::chrono::duration<long, std::ratio<1, 1000000000>>::period>
f69(const std::chrono::system_clock::duration &a0) {
  return a0.operator-();
}

std::chrono::system_clock::duration &
f70(std::chrono::system_clock::duration &a0) {
  return a0.operator++();
}

std::chrono::system_clock::duration f71(std::chrono::system_clock::duration &a0,
                                        int a1) {
  return a0.operator++(a1);
}

std::chrono::system_clock::duration &
f72(std::chrono::system_clock::duration &a0) {
  return a0.operator--();
}

std::chrono::system_clock::duration f73(std::chrono::system_clock::duration &a0,
                                        int a1) {
  return a0.operator--(a1);
}

std::chrono::system_clock::duration &
f74(std::chrono::system_clock::duration &a0,
    const std::chrono::system_clock::duration &a1) {
  return a0.operator+=(a1);
}

std::chrono::system_clock::duration &
f75(std::chrono::system_clock::duration &a0,
    const std::chrono::system_clock::duration &a1) {
  return a0.operator-=(a1);
}

std::chrono::system_clock::duration &
f76(std::chrono::system_clock::duration &a0,
    const std::chrono::duration<long, std::ratio<1, 1000000000>>::rep &a1) {
  return a0.operator*=(a1);
}

std::chrono::system_clock::duration &
f77(std::chrono::system_clock::duration &a0,
    const std::chrono::duration<long, std::ratio<1, 1000000000>>::rep &a1) {
  return a0.operator/=(a1);
}

std::chrono::system_clock::duration f78() {
  return std::chrono::duration<long, std::ratio<1, 1000000000>>::zero();
}

std::chrono::system_clock::duration f79() {
  return std::chrono::duration<long, std::ratio<1, 1000000000>>::min();
}

std::chrono::system_clock::duration f80() {
  return std::chrono::duration<long, std::ratio<1, 1000000000>>::max();
}

template <
    typename T1,
    typename T2 = typename std::enable_if<
        std::integral_constant<
            bool,
            std::is_convertible<const T1 &, std::int64_t>::value &&
                std::integral_constant<
                    bool, std::integral_constant<bool, false>::value ||
                              std::integral_constant<
                                  bool, !std::chrono::treat_as_floating_point<
                                            T1>::value>::value>::value>::value,
        void>::type>
std::chrono::system_clock::duration f81(const T1 &a0) {
  return std::chrono::system_clock::duration(a0);
}

template <
    typename T1 = std::chrono::duration<long, std::ratio<1, 1000000000>>::rep>
std::chrono::duration<long, std::ratio<1, 1000000000>> &
f82(std::chrono::system_clock::duration &a0,
    const std::chrono::duration<long, std::ratio<1, 1000000000>>::rep &a1) {
  return a0.operator%=(a1);
}

template <
    typename T1 = std::chrono::duration<long, std::ratio<1, 1000000000>>::rep>
std::chrono::duration<long, std::ratio<1, 1000000000>> &
f83(std::chrono::system_clock::duration &a0,
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

std::chrono::system_clock::time_point f84() {
  return std::chrono::system_clock::time_point();
}

std::chrono::system_clock::time_point
f85(const std::chrono::time_point<
    std::chrono::system_clock,
    std::chrono::duration<long, std::ratio<1, 1000000000>>>::duration &a0) {
  return std::chrono::system_clock::time_point(a0);
}

std::chrono::time_point<
    std::chrono::system_clock,
    std::chrono::duration<long, std::ratio<1, 1000000000>>>::duration
f86(const std::chrono::system_clock::time_point &a0) {
  return a0.time_since_epoch();
}

std::chrono::system_clock::time_point &
f87(std::chrono::system_clock::time_point &a0,
    const std::chrono::time_point<
        std::chrono::system_clock,
        std::chrono::duration<long, std::ratio<1, 1000000000>>>::duration &a1) {
  return a0.operator+=(a1);
}

std::chrono::system_clock::time_point &
f88(std::chrono::system_clock::time_point &a0,
    const std::chrono::time_point<
        std::chrono::system_clock,
        std::chrono::duration<long, std::ratio<1, 1000000000>>>::duration &a1) {
  return a0.operator-=(a1);
}

std::chrono::system_clock::time_point f89() {
  return std::chrono::time_point<
      std::chrono::system_clock,
      std::chrono::duration<long, std::ratio<1, 1000000000>>>::min();
}

std::chrono::system_clock::time_point f90() {
  return std::chrono::time_point<
      std::chrono::system_clock,
      std::chrono::duration<long, std::ratio<1, 1000000000>>>::max();
}

template <
    typename T1,
    typename T2 = typename std::enable_if<
        std::integral_constant<
            bool, std::is_convertible<
                      T1, std::chrono::system_clock::duration>::value>::value,
        void>::type>
std::chrono::system_clock::time_point
f91(const std::chrono::time_point<
    std::chrono::time_point<
        std::chrono::system_clock,
        std::chrono::duration<long, std::ratio<1, 1000000000>>>::clock,
    T1> &a0) {
  return std::chrono::system_clock::time_point(a0);
}

bool f92() { return std::chrono::system_clock::is_steady; }
