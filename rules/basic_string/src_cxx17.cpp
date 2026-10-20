#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <initializer_list>
#include <string>
#include <utility>

template <typename T1, typename T2, typename T3>
std::basic_string_view<T1, T2> f207(const std::basic_string<T1, T2, T3> &a0) {
  return static_cast<std::basic_string_view<T1, T2>>(a0);
}

template <typename T1, typename T2, typename T3>
T1 *f208(std::basic_string<T1, T2, T3> &a0) {
  return a0.data();
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<T1, T2, T3>
f209(const T4 &a0, typename std::basic_string<T1, T2, T3>::size_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2, const T3 &a3) {
  return std::basic_string<T1, T2, T3>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<T1, T2, T3> f210(const T4 &a0, const T3 &a1) {
  return std::basic_string<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<T1, T2, T3> &f211(std::basic_string<T1, T2, T3> &a0,
                                    const T4 &a1) {
  return a0.template operator= <T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<T1, T2, T3> &f212(std::basic_string<T1, T2, T3> &a0,
                                    const T4 &a1) {
  return a0.template operator+= <T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<T1, T2, T3> &f213(std::basic_string<T1, T2, T3> &a0,
                                    const T4 &a1) {
  return a0.template append<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<T1, T2, T3> &
f214(std::basic_string<T1, T2, T3> &a0, const T4 &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.template append<T4>(a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<T1, T2, T3> &f215(std::basic_string<T1, T2, T3> &a0,
                                    const T4 &a1) {
  return a0.template assign<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<T1, T2, T3> &
f216(std::basic_string<T1, T2, T3> &a0, const T4 &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.template assign<T4>(a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<T1, T2, T3> &
f217(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1, const T4 &a2) {
  return a0.template insert<T4>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<T1, T2, T3> &
f218(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1, const T4 &a2,
     typename std::basic_string<T1, T2, T3>::size_type a3,
     typename std::basic_string<T1, T2, T3>::size_type a4) {
  return a0.template insert<T4>(a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<T1, T2, T3> &
f219(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2, const T4 &a3) {
  return a0.template replace<T4>(a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<T1, T2, T3> &
f220(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2, const T4 &a3,
     typename std::basic_string<T1, T2, T3>::size_type a4,
     typename std::basic_string<T1, T2, T3>::size_type a5) {
  return a0.template replace<T4>(a1, a2, a3, a4, a5);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<T1, T2, T3> &
f221(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1,
     typename std::basic_string<T1, T2, T3>::const_iterator a2, const T4 &a3) {
  return a0.template replace<T4>(a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::basic_string<T1, T2, T3>::size_type
f222(const std::basic_string<T1, T2, T3> &a0, const T4 &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.template find<T4>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::basic_string<T1, T2, T3>::size_type
f223(const std::basic_string<T1, T2, T3> &a0, const T4 &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.template rfind<T4>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::basic_string<T1, T2, T3>::size_type
f224(const std::basic_string<T1, T2, T3> &a0, const T4 &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.template find_first_of<T4>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::basic_string<T1, T2, T3>::size_type
f225(const std::basic_string<T1, T2, T3> &a0, const T4 &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.template find_last_of<T4>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::basic_string<T1, T2, T3>::size_type
f226(const std::basic_string<T1, T2, T3> &a0, const T4 &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.template find_first_not_of<T4>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
typename std::basic_string<T1, T2, T3>::size_type
f227(const std::basic_string<T1, T2, T3> &a0, const T4 &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.template find_last_not_of<T4>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
int f228(const std::basic_string<T1, T2, T3> &a0, const T4 &a1) {
  return a0.template compare<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
int f229(const std::basic_string<T1, T2, T3> &a0,
         typename std::basic_string<T1, T2, T3>::size_type a1,
         typename std::basic_string<T1, T2, T3>::size_type a2, const T4 &a3) {
  return a0.template compare<T4>(a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
int f230(const std::basic_string<T1, T2, T3> &a0,
         typename std::basic_string<T1, T2, T3>::size_type a1,
         typename std::basic_string<T1, T2, T3>::size_type a2, const T4 &a3,
         typename std::basic_string<T1, T2, T3>::size_type a4,
         typename std::basic_string<T1, T2, T3>::size_type a5) {
  return a0.template compare<T4>(a1, a2, a3, a4, a5);
}
