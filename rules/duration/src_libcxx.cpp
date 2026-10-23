#include <chrono>
#include <utility>

template <typename T1, typename T2>
typename std::common_type<std::chrono::duration<T1, T2>>::type
f22(const std::chrono::duration<T1, T2> &a0) {
  return a0.operator+();
}

template <typename T1, typename T2>
typename std::common_type<std::chrono::duration<T1, T2>>::type
f23(const std::chrono::duration<T1, T2> &a0) {
  return a0.operator-();
}

template <typename T1, typename T2>
std::chrono::duration<T1, T2> &f24(std::chrono::duration<T1, T2> &a0,
                                   const T1 &a1) {
  return a0.operator%=(a1);
}

template <typename T1, typename T2>
std::chrono::duration<T1, T2> &f25(std::chrono::duration<T1, T2> &a0,
                                   const std::chrono::duration<T1, T2> &a1) {
  return a0.operator%=(a1);
}

template <
    typename T1, typename T2, typename T3,
    std::__enable_if_t<std::is_convertible<const T3 &, T1>::value &&
                           (std::chrono::treat_as_floating_point<T1>::value ||
                            !std::chrono::treat_as_floating_point<T3>::value),
                       int>
        T4 = 0>
std::chrono::duration<T1, T2> f26(const T3 &a0) {
  return std::chrono::duration<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3, typename T4,
          std::__enable_if_t<
              std::chrono::duration<T1, T2>::template __no_overflow<
                  T4, typename T2::type>::value &&
                  (std::chrono::treat_as_floating_point<T1>::value ||
                   (std::chrono::duration<T1, T2>::template __no_overflow<
                        T4, typename T2::type>::type::den == 1 &&
                    !std::chrono::treat_as_floating_point<T3>::value)),
              int>
              T5 = 0>
std::chrono::duration<T1, T2> f27(const std::chrono::duration<T3, T4> &a0) {
  return std::chrono::duration<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<std::chrono::__is_duration_v<T1>, int> T4 = 0>
T1 f28(const std::chrono::duration<T2, T3> &a0) {
  return std::chrono::duration_cast<T1, T2, T3>(a0);
}

using t4 = std::chrono::hours;

using t5 = std::chrono::microseconds;

using t6 = std::chrono::milliseconds;

using t7 = std::chrono::minutes;

using t8 = std::chrono::nanoseconds;

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<
              !std::chrono::__is_duration_v<T3> &&
                  std::is_convertible<const T3 &, typename std::common_type<
                                                      T1, T3>::type>::value,
              int>
              T4 = 0>
std::chrono::duration<typename std::common_type<T1, T3>::type, T2>
f29(const std::chrono::duration<T1, T2> &a0, const T3 &a1) {
  return std::chrono::operator% <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::common_type<std::chrono::duration<T1, T2>,
                          std::chrono::duration<T3, T4>>::type
f30(const std::chrono::duration<T1, T2> &a0,
    const std::chrono::duration<T3, T4> &a1) {
  return std::chrono::operator% <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<
              std::is_convertible<
                  const T1 &, typename std::common_type<T1, T3>::type>::value,
              int>
              T4 = 0>
std::chrono::duration<typename std::common_type<T1, T3>::type, T2>
f31(const T1 &a0, const std::chrono::duration<T3, T2> &a1) {
  return std::chrono::operator* <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<
              std::is_convertible<
                  const T3 &, typename std::common_type<T1, T3>::type>::value,
              int>
              T4 = 0>
std::chrono::duration<typename std::common_type<T1, T3>::type, T2>
f32(const std::chrono::duration<T1, T2> &a0, const T3 &a1) {
  return std::chrono::operator* <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::common_type<std::chrono::duration<T1, T2>,
                          std::chrono::duration<T3, T4>>::type
f33(const std::chrono::duration<T1, T2> &a0,
    const std::chrono::duration<T3, T4> &a1) {
  return std::chrono::operator+ <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::common_type<std::chrono::duration<T1, T2>,
                          std::chrono::duration<T3, T4>>::type
f34(const std::chrono::duration<T1, T2> &a0,
    const std::chrono::duration<T3, T4> &a1) {
  return std::chrono::operator- <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<
              !std::chrono::__is_duration_v<T3> &&
                  std::is_convertible<const T3 &, typename std::common_type<
                                                      T1, T3>::type>::value,
              int>
              T4 = 0>
std::chrono::duration<typename std::common_type<T1, T3>::type, T2>
f35(const std::chrono::duration<T1, T2> &a0, const T3 &a1) {
  return std::chrono::operator/ <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::common_type<T1, T3>::type
f36(const std::chrono::duration<T1, T2> &a0,
    const std::chrono::duration<T3, T4> &a1) {
  return std::chrono::operator/ <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f37(const std::chrono::duration<T1, T2> &a0,
         const std::chrono::duration<T3, T4> &a1) {
  return std::chrono::operator< <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f38(const std::chrono::duration<T1, T2> &a0,
         const std::chrono::duration<T3, T4> &a1) {
  return std::chrono::operator<= <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f39(const std::chrono::duration<T1, T2> &a0,
         const std::chrono::duration<T3, T4> &a1) {
  return std::chrono::operator== <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f40(const std::chrono::duration<T1, T2> &a0,
         const std::chrono::duration<T3, T4> &a1) {
  return std::chrono::operator><T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f41(const std::chrono::duration<T1, T2> &a0,
         const std::chrono::duration<T3, T4> &a1) {
  return std::chrono::operator>= <T1, T2, T3, T4>(a0, a1);
}

using t9 = std::chrono::seconds;
