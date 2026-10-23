#include <charconv>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

using t1 = std::chars_format;

std::chars_format f1() { return std::chars_format::scientific; }

std::chars_format f2() { return std::chars_format::fixed; }

std::chars_format f3() { return std::chars_format::hex; }

std::chars_format f4() { return std::chars_format::general; }

std::from_chars_result f5(const char *a0, const char *a1, long double &a2,
                          std::chars_format a3) {
  return std::from_chars(a0, a1, a2, a3);
}

std::from_chars_result f6(const char *a0, const char *a1, double &a2,
                          std::chars_format a3) {
  return std::from_chars(a0, a1, a2, a3);
}

std::from_chars_result f7(const char *a0, const char *a1, float &a2,
                          std::chars_format a3) {
  return std::from_chars(a0, a1, a2, a3);
}

template <
    typename T1,
    std::enable_if_t<
        std::integral_constant<
            bool,
            std::integral_constant<
                bool,
                std::integral_constant<
                    bool, std::is_same<typename std::remove_cv<T1>::type,
                                       signed char>::value ||
                              std::is_same<typename std::remove_cv<T1>::type,
                                           short>::value ||
                              std::is_same<typename std::remove_cv<T1>::type,
                                           int>::value ||
                              std::is_same<typename std::remove_cv<T1>::type,
                                           long>::value ||
                              std::is_same<typename std::remove_cv<T1>::type,
                                           long long>::value>::value ||
                    std::integral_constant<
                        bool,
                        std::is_same<typename std::remove_cv<T1>::type,
                                     unsigned char>::value ||
                            std::is_same<typename std::remove_cv<T1>::type,
                                         unsigned short>::value ||
                            std::is_same<typename std::remove_cv<T1>::type,
                                         unsigned int>::value ||
                            std::is_same<typename std::remove_cv<T1>::type,
                                         unsigned long>::value ||
                            std::is_same<typename std::remove_cv<T1>::type,
                                         unsigned long long>::value>::value>::
                    value ||
                std::is_same<char, std::remove_cv_t<T1>>::value>::value,
        int>
        T2 = 0>
std::from_chars_result f8(const char *a0, const char *a1, T1 &a2, int a3) {
  return std::from_chars<T1>(a0, a1, a2, a3);
}

using t2 = std::from_chars_result;

std::from_chars_result f9() { return std::from_chars_result(); }

std::from_chars_result f10(const std::from_chars_result &a0) {
  return std::from_chars_result(a0);
}

std::from_chars_result f11(std::from_chars_result &&a0) {
  return std::from_chars_result(std::move(a0));
}

std::from_chars_result &f12(std::from_chars_result &a0,
                            const std::from_chars_result &a1) {
  return a0.operator=(a1);
}

std::from_chars_result &f13(std::from_chars_result &a0,
                            std::from_chars_result &&a1) {
  return a0.operator=(std::move(a1));
}

std::chars_format f14(std::chars_format a0, std::chars_format a1) {
  return a0 & a1;
}

std::chars_format &f15(std::chars_format &a0, std::chars_format a1) {
  return std::operator&=(a0, a1);
}

std::chars_format f16(std::chars_format a0, std::chars_format a1) {
  return a0 ^ a1;
}

std::chars_format &f17(std::chars_format &a0, std::chars_format a1) {
  return std::operator^=(a0, a1);
}

std::chars_format f18(std::chars_format a0, std::chars_format a1) {
  return a0 | a1;
}

std::chars_format &f19(std::chars_format &a0, std::chars_format a1) {
  return std::operator|=(a0, a1);
}

std::chars_format f20(std::chars_format a0) { return ~a0; }

std::to_chars_result f21(char *a0, char *a1, char a2, int a3) {
  return std::to_chars(a0, a1, a2, a3);
}

std::to_chars_result f22(char *a0, char *a1, long double a2) {
  return std::to_chars(a0, a1, a2);
}

std::to_chars_result f23(char *a0, char *a1, long double a2,
                         std::chars_format a3) {
  return std::to_chars(a0, a1, a2, a3);
}

std::to_chars_result f24(char *a0, char *a1, long double a2,
                         std::chars_format a3, int a4) {
  return std::to_chars(a0, a1, a2, a3, a4);
}

std::to_chars_result f25(char *a0, char *a1, int a2, int a3) {
  return std::to_chars(a0, a1, a2, a3);
}

std::to_chars_result f26(char *a0, char *a1, long long a2, int a3) {
  return std::to_chars(a0, a1, a2, a3);
}

std::to_chars_result f27(char *a0, char *a1, long a2, int a3) {
  return std::to_chars(a0, a1, a2, a3);
}

std::to_chars_result f28(char *a0, char *a1, short a2, int a3) {
  return std::to_chars(a0, a1, a2, a3);
}

std::to_chars_result f29(char *a0, char *a1, unsigned char a2, int a3) {
  return std::to_chars(a0, a1, a2, a3);
}

std::to_chars_result f30(char *a0, char *a1, double a2) {
  return std::to_chars(a0, a1, a2);
}

std::to_chars_result f31(char *a0, char *a1, double a2, std::chars_format a3) {
  return std::to_chars(a0, a1, a2, a3);
}

std::to_chars_result f32(char *a0, char *a1, double a2, std::chars_format a3,
                         int a4) {
  return std::to_chars(a0, a1, a2, a3, a4);
}

std::to_chars_result f33(char *a0, char *a1, float a2) {
  return std::to_chars(a0, a1, a2);
}

std::to_chars_result f34(char *a0, char *a1, float a2, std::chars_format a3) {
  return std::to_chars(a0, a1, a2, a3);
}

std::to_chars_result f35(char *a0, char *a1, float a2, std::chars_format a3,
                         int a4) {
  return std::to_chars(a0, a1, a2, a3, a4);
}

std::to_chars_result f36(char *a0, char *a1, unsigned int a2, int a3) {
  return std::to_chars(a0, a1, a2, a3);
}

std::to_chars_result f37(char *a0, char *a1, unsigned long long a2, int a3) {
  return std::to_chars(a0, a1, a2, a3);
}

std::to_chars_result f38(char *a0, char *a1, unsigned long a2, int a3) {
  return std::to_chars(a0, a1, a2, a3);
}

std::to_chars_result f39(char *a0, char *a1, signed char a2, int a3) {
  return std::to_chars(a0, a1, a2, a3);
}

std::to_chars_result f40(char *a0, char *a1, unsigned short a2, int a3) {
  return std::to_chars(a0, a1, a2, a3);
}

using t3 = std::to_chars_result;

std::to_chars_result f41() { return std::to_chars_result(); }

std::to_chars_result f42(const std::to_chars_result &a0) {
  return std::to_chars_result(a0);
}

std::to_chars_result f43(std::to_chars_result &&a0) {
  return std::to_chars_result(std::move(a0));
}

std::to_chars_result &f44(std::to_chars_result &a0,
                          const std::to_chars_result &a1) {
  return a0.operator=(a1);
}

std::to_chars_result &f45(std::to_chars_result &a0, std::to_chars_result &&a1) {
  return a0.operator=(std::move(a1));
}
