#include <chrono>
#include <ratio>
#include <utility>

using t1 = std::chrono::steady_clock;

using t2 = std::chrono::steady_clock::duration;

using t3 = std::chrono::steady_clock::rep;

using t4 = std::chrono::steady_clock::period;

using t5 = std::chrono::steady_clock::time_point;

std::chrono::steady_clock::time_point f1() {
  return std::chrono::steady_clock::now();
}

std::chrono::steady_clock f2() { return std::chrono::steady_clock(); }

std::chrono::steady_clock f3(const std::chrono::steady_clock &a0) {
  return std::chrono::steady_clock(a0);
}

std::chrono::steady_clock f4(std::chrono::steady_clock &&a0) {
  return std::chrono::steady_clock(std::move(a0));
}

std::chrono::steady_clock &f5(std::chrono::steady_clock &a0,
                              const std::chrono::steady_clock &a1) {
  return a0.operator=(a1);
}

std::chrono::steady_clock &f6(std::chrono::steady_clock &a0,
                              std::chrono::steady_clock &&a1) {
  return a0.operator=(std::move(a1));
}

using t6 = std::chrono::duration<long long, std::ratio<1, 1000000000>>::rep;

using t7 = std::chrono::duration<long long, std::ratio<1, 1000000000>>::period;

std::chrono::steady_clock::duration f7() {
  return std::chrono::steady_clock::duration();
}

std::chrono::duration<long long, std::ratio<1, 1000000000>>::rep
f8(const std::chrono::steady_clock::duration &a0) {
  return a0.count();
}

typename std::common_type<std::chrono::steady_clock::duration>::type
f9(const std::chrono::steady_clock::duration &a0) {
  return a0.operator+();
}

typename std::common_type<std::chrono::steady_clock::duration>::type
f10(const std::chrono::steady_clock::duration &a0) {
  return a0.operator-();
}

std::chrono::steady_clock::duration &
f11(std::chrono::steady_clock::duration &a0) {
  return a0.operator++();
}

std::chrono::steady_clock::duration f12(std::chrono::steady_clock::duration &a0,
                                        int a1) {
  return a0.operator++(a1);
}

std::chrono::steady_clock::duration &
f13(std::chrono::steady_clock::duration &a0) {
  return a0.operator--();
}

std::chrono::steady_clock::duration f14(std::chrono::steady_clock::duration &a0,
                                        int a1) {
  return a0.operator--(a1);
}

std::chrono::steady_clock::duration &
f15(std::chrono::steady_clock::duration &a0,
    const std::chrono::steady_clock::duration &a1) {
  return a0.operator+=(a1);
}

std::chrono::steady_clock::duration &
f16(std::chrono::steady_clock::duration &a0,
    const std::chrono::steady_clock::duration &a1) {
  return a0.operator-=(a1);
}

std::chrono::steady_clock::duration &
f17(std::chrono::steady_clock::duration &a0,
    const std::chrono::duration<long long, std::ratio<1, 1000000000>>::rep
        &a1) {
  return a0.operator*=(a1);
}

std::chrono::steady_clock::duration &
f18(std::chrono::steady_clock::duration &a0,
    const std::chrono::duration<long long, std::ratio<1, 1000000000>>::rep
        &a1) {
  return a0.operator/=(a1);
}

std::chrono::steady_clock::duration &
f19(std::chrono::steady_clock::duration &a0,
    const std::chrono::duration<long long, std::ratio<1, 1000000000>>::rep
        &a1) {
  return a0.operator%=(a1);
}

std::chrono::steady_clock::duration &
f20(std::chrono::steady_clock::duration &a0,
    const std::chrono::steady_clock::duration &a1) {
  return a0.operator%=(a1);
}

std::chrono::steady_clock::duration f21() {
  return std::chrono::duration<long long, std::ratio<1, 1000000000>>::zero();
}

std::chrono::steady_clock::duration f22() {
  return std::chrono::duration<long long, std::ratio<1, 1000000000>>::min();
}

std::chrono::steady_clock::duration f23() {
  return std::chrono::duration<long long, std::ratio<1, 1000000000>>::max();
}

template <typename T1,
          typename std::enable_if<
              std::is_convertible<const T1 &, long long>::value &&
                  (std::integral_constant<bool, false>::value ||
                   !std::chrono::treat_as_floating_point<T1>::value),
              int>::type T2 = 0>
std::chrono::steady_clock::duration f24(const T1 &a0) {
  return std::chrono::steady_clock::duration(a0);
}

using t8 = std::ratio<1, 1000000000>::type;

using t9 = std::chrono::time_point<
    std::chrono::steady_clock,
    std::chrono::duration<long long, std::ratio<1, 1000000000>>>::clock;

using t10 = std::chrono::time_point<
    std::chrono::steady_clock,
    std::chrono::duration<long long, std::ratio<1, 1000000000>>>::duration;

using t11 = std::chrono::time_point<
    std::chrono::steady_clock,
    std::chrono::duration<long long, std::ratio<1, 1000000000>>>::rep;

using t12 = std::chrono::time_point<
    std::chrono::steady_clock,
    std::chrono::duration<long long, std::ratio<1, 1000000000>>>::period;

std::chrono::steady_clock::time_point f25() {
  return std::chrono::steady_clock::time_point();
}

std::chrono::steady_clock::time_point
f26(const std::chrono::time_point<
    std::chrono::steady_clock,
    std::chrono::duration<long long, std::ratio<1, 1000000000>>>::duration
        &a0) {
  return std::chrono::steady_clock::time_point(a0);
}

std::chrono::time_point<
    std::chrono::steady_clock,
    std::chrono::duration<long long, std::ratio<1, 1000000000>>>::duration
f27(const std::chrono::steady_clock::time_point &a0) {
  return a0.time_since_epoch();
}

std::chrono::steady_clock::time_point &
f28(std::chrono::steady_clock::time_point &a0,
    const std::chrono::time_point<
        std::chrono::steady_clock,
        std::chrono::duration<long long, std::ratio<1, 1000000000>>>::duration
        &a1) {
  return a0.operator+=(a1);
}

std::chrono::steady_clock::time_point &
f29(std::chrono::steady_clock::time_point &a0,
    const std::chrono::time_point<
        std::chrono::steady_clock,
        std::chrono::duration<long long, std::ratio<1, 1000000000>>>::duration
        &a1) {
  return a0.operator-=(a1);
}

std::chrono::steady_clock::time_point f30() {
  return std::chrono::time_point<
      std::chrono::steady_clock,
      std::chrono::duration<long long, std::ratio<1, 1000000000>>>::min();
}

std::chrono::steady_clock::time_point f31() {
  return std::chrono::time_point<
      std::chrono::steady_clock,
      std::chrono::duration<long long, std::ratio<1, 1000000000>>>::max();
}

template <
    typename T1,
    typename std::enable_if<
        std::is_convertible<T1, std::chrono::steady_clock::duration>::value,
        int>::type T2 = 0>
std::chrono::steady_clock::time_point
f32(const std::chrono::time_point<
    std::chrono::time_point<
        std::chrono::steady_clock,
        std::chrono::duration<long long, std::ratio<1, 1000000000>>>::clock,
    T1> &a0) {
  return std::chrono::steady_clock::time_point(a0);
}

bool f33() { return std::chrono::steady_clock::is_steady; }
