#include <chrono>
#include <type_traits>
#include <utility>

template <typename T1, typename T2, typename T3>
std::chrono::__enable_if_is_duration<T1>
f1(const std::chrono::duration<T2, T3> &a0) {
  return std::chrono::duration_cast<T1, T2, T3>(a0);
}

using t1 = std::chrono::high_resolution_clock;

using t2 = std::chrono::hours;

using t3 = std::chrono::microseconds;

using t4 = std::chrono::milliseconds;

using t5 = std::chrono::minutes;

using t6 = std::chrono::nanoseconds;

template <typename T1, typename T2, typename T3>
std::chrono::duration<
    std::chrono::__common_rep_t<T1, std::chrono::__disable_if_is_duration<T3>>,
    T2>
f2(const std::chrono::duration<T1, T2> &a0, const T3 &a1) {
  return std::chrono::operator% <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::common_type<std::chrono::duration<T1, T2>,
                          std::chrono::duration<T3, T4>>::type
f3(const std::chrono::duration<T1, T2> &a0,
   const std::chrono::duration<T3, T4> &a1) {
  return std::chrono::operator% <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::chrono::duration<std::chrono::__common_rep_t<T1, T3>, T2>
f4(const std::chrono::duration<T1, T2> &a0, const T3 &a1) {
  return std::chrono::operator* <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::chrono::duration<std::chrono::__common_rep_t<T2, T1>, T3>
f5(const T1 &a0, const std::chrono::duration<T2, T3> &a1) {
  return std::chrono::operator* <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::common_type<std::chrono::duration<T1, T2>,
                          std::chrono::duration<T3, T4>>::type
f6(const std::chrono::duration<T1, T2> &a0,
   const std::chrono::duration<T3, T4> &a1) {
  return std::chrono::operator+ <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::chrono::time_point<
    T3, typename std::common_type<std::chrono::duration<T1, T2>, T4>::type>
f7(const std::chrono::duration<T1, T2> &a0,
   const std::chrono::time_point<T3, T4> &a1) {
  return std::chrono::operator+ <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::chrono::time_point<
    T1, typename std::common_type<T2, std::chrono::duration<T3, T4>>::type>
f8(const std::chrono::time_point<T1, T2> &a0,
   const std::chrono::duration<T3, T4> &a1) {
  return std::chrono::operator+ <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3>
typename std::common_type<T2, T3>::type
f9(const std::chrono::time_point<T1, T2> &a0,
   const std::chrono::time_point<T1, T3> &a1) {
  return std::chrono::operator- <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::common_type<std::chrono::duration<T1, T2>,
                          std::chrono::duration<T3, T4>>::type
f10(const std::chrono::duration<T1, T2> &a0,
    const std::chrono::duration<T3, T4> &a1) {
  return std::chrono::operator- <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::chrono::time_point<
    T1, typename std::common_type<T2, std::chrono::duration<T3, T4>>::type>
f11(const std::chrono::time_point<T1, T2> &a0,
    const std::chrono::duration<T3, T4> &a1) {
  return std::chrono::operator- <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::chrono::duration<
    std::chrono::__common_rep_t<T1, std::chrono::__disable_if_is_duration<T3>>,
    T2>
f12(const std::chrono::duration<T1, T2> &a0, const T3 &a1) {
  return std::chrono::operator/ <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::common_type<T1, T3>::type
f13(const std::chrono::duration<T1, T2> &a0,
    const std::chrono::duration<T3, T4> &a1) {
  return std::chrono::operator/ <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f14(const std::chrono::time_point<T1, T2> &a0,
         const std::chrono::time_point<T1, T3> &a1) {
  return std::chrono::operator< <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f15(const std::chrono::duration<T1, T2> &a0,
         const std::chrono::duration<T3, T4> &a1) {
  return std::chrono::operator< <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f16(const std::chrono::time_point<T1, T2> &a0,
         const std::chrono::time_point<T1, T3> &a1) {
  return std::chrono::operator<= <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f17(const std::chrono::duration<T1, T2> &a0,
         const std::chrono::duration<T3, T4> &a1) {
  return std::chrono::operator<= <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f18(const std::chrono::time_point<T1, T2> &a0,
         const std::chrono::time_point<T1, T3> &a1) {
  return std::chrono::operator== <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f19(const std::chrono::duration<T1, T2> &a0,
         const std::chrono::duration<T3, T4> &a1) {
  return std::chrono::operator== <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f20(const std::chrono::time_point<T1, T2> &a0,
         const std::chrono::time_point<T1, T3> &a1) {
  return std::chrono::operator><T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f21(const std::chrono::duration<T1, T2> &a0,
         const std::chrono::duration<T3, T4> &a1) {
  return std::chrono::operator><T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f22(const std::chrono::time_point<T1, T2> &a0,
         const std::chrono::time_point<T1, T3> &a1) {
  return std::chrono::operator>= <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f23(const std::chrono::duration<T1, T2> &a0,
         const std::chrono::duration<T3, T4> &a1) {
  return std::chrono::operator>= <T1, T2, T3, T4>(a0, a1);
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

bool f30() { return std::chrono::steady_clock::is_steady; }

using t13 = std::chrono::system_clock;

using t14 = std::chrono::system_clock::duration;

using t15 = std::chrono::system_clock::rep;

using t16 = std::chrono::system_clock::period;

using t17 = std::chrono::system_clock::time_point;

std::chrono::system_clock::time_point f31() {
  return std::chrono::system_clock::now();
}

std::time_t f32(const std::chrono::system_clock::time_point &a0) {
  return std::chrono::system_clock::to_time_t(a0);
}

std::chrono::system_clock::time_point f33(std::time_t a0) {
  return std::chrono::system_clock::from_time_t(a0);
}

std::chrono::system_clock f34() { return std::chrono::system_clock(); }

std::chrono::system_clock f35(const std::chrono::system_clock &a0) {
  return std::chrono::system_clock(a0);
}

std::chrono::system_clock f36(std::chrono::system_clock &&a0) {
  return std::chrono::system_clock(std::move(a0));
}

std::chrono::system_clock &f37(std::chrono::system_clock &a0,
                               const std::chrono::system_clock &a1) {
  return a0.operator=(a1);
}

std::chrono::system_clock &f38(std::chrono::system_clock &a0,
                               std::chrono::system_clock &&a1) {
  return a0.operator=(std::move(a1));
}

bool f39() { return std::chrono::system_clock::is_steady; }

template <typename T1, typename T2, typename T3>
std::__enable_if_t<std::chrono::__is_duration<T1>::value,
                   std::chrono::time_point<T2, T1>>
f40(const std::chrono::time_point<T2, T3> &a0) {
  return std::chrono::time_point_cast<T1, T2, T3>(a0);
}
