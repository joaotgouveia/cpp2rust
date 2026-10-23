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
  return std::chrono::operator% <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::common_type<std::chrono::duration<T1, T2>,
                          std::chrono::duration<T3, T4>>::type
f2(const std::chrono::duration<T1, T2> &a0,
   const std::chrono::duration<T3, T4> &a1) {
  return std::chrono::operator+ <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::chrono::time_point<
    T3, typename std::common_type<std::chrono::duration<T1, T2>, T4>::type>
f3(const std::chrono::duration<T1, T2> &a0,
   const std::chrono::time_point<T3, T4> &a1) {
  return std::chrono::operator+ <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::chrono::time_point<
    T1, typename std::common_type<T2, std::chrono::duration<T3, T4>>::type>
f4(const std::chrono::time_point<T1, T2> &a0,
   const std::chrono::duration<T3, T4> &a1) {
  return std::chrono::operator+ <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3>
typename std::common_type<T2, T3>::type
f5(const std::chrono::time_point<T1, T2> &a0,
   const std::chrono::time_point<T1, T3> &a1) {
  return std::chrono::operator- <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::common_type<std::chrono::duration<T1, T2>,
                          std::chrono::duration<T3, T4>>::type
f6(const std::chrono::duration<T1, T2> &a0,
   const std::chrono::duration<T3, T4> &a1) {
  return std::chrono::operator- <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::chrono::time_point<
    T1, typename std::common_type<T2, std::chrono::duration<T3, T4>>::type>
f7(const std::chrono::time_point<T1, T2> &a0,
   const std::chrono::duration<T3, T4> &a1) {
  return std::chrono::operator- <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::common_type<T1, T3>::type
f8(const std::chrono::duration<T1, T2> &a0,
   const std::chrono::duration<T3, T4> &a1) {
  return std::chrono::operator/ <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f9(const std::chrono::time_point<T1, T2> &a0,
        const std::chrono::time_point<T1, T3> &a1) {
  return std::chrono::operator< <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f10(const std::chrono::duration<T1, T2> &a0,
         const std::chrono::duration<T3, T4> &a1) {
  return std::chrono::operator< <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f11(const std::chrono::time_point<T1, T2> &a0,
         const std::chrono::time_point<T1, T3> &a1) {
  return std::chrono::operator<= <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f12(const std::chrono::duration<T1, T2> &a0,
         const std::chrono::duration<T3, T4> &a1) {
  return std::chrono::operator<= <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f13(const std::chrono::time_point<T1, T2> &a0,
         const std::chrono::time_point<T1, T3> &a1) {
  return std::chrono::operator== <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f14(const std::chrono::duration<T1, T2> &a0,
         const std::chrono::duration<T3, T4> &a1) {
  return std::chrono::operator== <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f15(const std::chrono::time_point<T1, T2> &a0,
         const std::chrono::time_point<T1, T3> &a1) {
  return std::chrono::operator><T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f16(const std::chrono::duration<T1, T2> &a0,
         const std::chrono::duration<T3, T4> &a1) {
  return std::chrono::operator><T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f17(const std::chrono::time_point<T1, T2> &a0,
         const std::chrono::time_point<T1, T3> &a1) {
  return std::chrono::operator>= <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f18(const std::chrono::duration<T1, T2> &a0,
         const std::chrono::duration<T3, T4> &a1) {
  return std::chrono::operator>= <T1, T2, T3, T4>(a0, a1);
}

using t7 = std::chrono::seconds;

using t8 = std::chrono::steady_clock;

using t9 = std::chrono::steady_clock::duration;

using t10 = std::chrono::steady_clock::rep;

using t11 = std::chrono::steady_clock::period;

using t12 = std::chrono::steady_clock::time_point;

std::chrono::steady_clock::time_point f19() {
  return std::chrono::steady_clock::now();
}

std::chrono::steady_clock f20() { return std::chrono::steady_clock(); }

std::chrono::steady_clock f21(const std::chrono::steady_clock &a0) {
  return std::chrono::steady_clock(a0);
}

std::chrono::steady_clock f22(std::chrono::steady_clock &&a0) {
  return std::chrono::steady_clock(std::move(a0));
}

std::chrono::steady_clock &f23(std::chrono::steady_clock &a0,
                               const std::chrono::steady_clock &a1) {
  return a0.operator=(a1);
}

std::chrono::steady_clock &f24(std::chrono::steady_clock &a0,
                               std::chrono::steady_clock &&a1) {
  return a0.operator=(std::move(a1));
}

bool f25() { return std::chrono::steady_clock::is_steady; }

using t13 = std::chrono::system_clock;

using t14 = std::chrono::system_clock::duration;

using t15 = std::chrono::system_clock::rep;

using t16 = std::chrono::system_clock::period;

using t17 = std::chrono::system_clock::time_point;

std::chrono::system_clock::time_point f26() {
  return std::chrono::system_clock::now();
}

std::time_t f27(const std::chrono::system_clock::time_point &a0) {
  return std::chrono::system_clock::to_time_t(a0);
}

std::chrono::system_clock::time_point f28(std::time_t a0) {
  return std::chrono::system_clock::from_time_t(a0);
}

std::chrono::system_clock f29() { return std::chrono::system_clock(); }

std::chrono::system_clock f30(const std::chrono::system_clock &a0) {
  return std::chrono::system_clock(a0);
}

std::chrono::system_clock f31(std::chrono::system_clock &&a0) {
  return std::chrono::system_clock(std::move(a0));
}

std::chrono::system_clock &f32(std::chrono::system_clock &a0,
                               const std::chrono::system_clock &a1) {
  return a0.operator=(a1);
}

std::chrono::system_clock &f33(std::chrono::system_clock &a0,
                               std::chrono::system_clock &&a1) {
  return a0.operator=(std::move(a1));
}

bool f34() { return std::chrono::system_clock::is_steady; }
