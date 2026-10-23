#include <algorithm>
#include <initializer_list>
#include <string>
#include <type_traits>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2, typename T3>
std::basic_string_view<T1, T2> f215(const std::basic_string<T1, T2, T3> &a0) {
  return static_cast<std::basic_string_view<T1, T2>>(a0);
}

template <typename T1, typename T2, typename T3>
T1 *f216(std::basic_string<T1, T2, T3> &a0) {
  return a0.data();
}

template <typename T1, typename T2, typename T3, typename T4,
          typename T5 = std::enable_if_t<std::is_convertible_v<
              const T4 &, std::basic_string_view<T1, T2>>>>
std::basic_string<T1, T2, T3>
f217(const T4 &a0, typename std::basic_string<T1, T2, T3>::size_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2, const T3 &a3) {
  return std::basic_string<T1, T2, T3>(a0, a1, a2, a3);
}

template <
    typename T1, typename T2, typename T3, typename T4,
    typename T5 = std::enable_if_t<
        std::integral_constant<
            bool,
            std::is_convertible<T4 const &,
                                std::basic_string_view<T1, T2>>::value &&
                std::integral_constant<
                    bool, !std::is_convertible<
                              T4 const *, const std::basic_string<T1, T2, T3>
                                              *>::value>::value &&
                std::integral_constant<
                    bool, !std::is_convertible<T4 const &, const T1 *>::value>::
                    value>::value,
        void>>
std::basic_string<T1, T2, T3> f218(const T4 &a0, const T3 &a1) {
  return std::basic_string<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::enable_if_t<
    std::integral_constant<
        bool, std::is_convertible<T4 const &,
                                  std::basic_string_view<T1, T2>>::value &&
                  std::integral_constant<
                      bool, !std::is_convertible<
                                T4 const *, const std::basic_string<T1, T2, T3>
                                                *>::value>::value &&
                  std::integral_constant<
                      bool, !std::is_convertible<
                                T4 const &, const T1 *>::value>::value>::value,
    std::basic_string<T1, T2, T3> &>
f219(std::basic_string<T1, T2, T3> &a0, const T4 &a1) {
  return a0.template operator= <T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::enable_if_t<
    std::integral_constant<
        bool, std::is_convertible<T4 const &,
                                  std::basic_string_view<T1, T2>>::value &&
                  std::integral_constant<
                      bool, !std::is_convertible<
                                T4 const *, const std::basic_string<T1, T2, T3>
                                                *>::value>::value &&
                  std::integral_constant<
                      bool, !std::is_convertible<
                                T4 const &, const T1 *>::value>::value>::value,
    std::basic_string<T1, T2, T3> &>
f220(std::basic_string<T1, T2, T3> &a0, const T4 &a1) {
  return a0.template operator+= <T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::enable_if_t<
    std::integral_constant<
        bool, std::is_convertible<T4 const &,
                                  std::basic_string_view<T1, T2>>::value &&
                  std::integral_constant<
                      bool, !std::is_convertible<
                                T4 const *, const std::basic_string<T1, T2, T3>
                                                *>::value>::value &&
                  std::integral_constant<
                      bool, !std::is_convertible<
                                T4 const &, const T1 *>::value>::value>::value,
    std::basic_string<T1, T2, T3> &>
f221(std::basic_string<T1, T2, T3> &a0, const T4 &a1) {
  return a0.template append<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::enable_if_t<
    std::integral_constant<
        bool, std::is_convertible<T4 const &,
                                  std::basic_string_view<T1, T2>>::value &&
                  std::integral_constant<
                      bool, !std::is_convertible<
                                T4 const *, const std::basic_string<T1, T2, T3>
                                                *>::value>::value &&
                  std::integral_constant<
                      bool, !std::is_convertible<
                                T4 const &, const T1 *>::value>::value>::value,
    std::basic_string<T1, T2, T3> &>
f222(std::basic_string<T1, T2, T3> &a0, const T4 &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.template append<T4>(a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
std::enable_if_t<
    std::integral_constant<
        bool, std::is_convertible<T4 const &,
                                  std::basic_string_view<T1, T2>>::value &&
                  std::integral_constant<
                      bool, !std::is_convertible<
                                T4 const *, const std::basic_string<T1, T2, T3>
                                                *>::value>::value &&
                  std::integral_constant<
                      bool, !std::is_convertible<
                                T4 const &, const T1 *>::value>::value>::value,
    std::basic_string<T1, T2, T3> &>
f223(std::basic_string<T1, T2, T3> &a0, const T4 &a1) {
  return a0.template assign<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::enable_if_t<
    std::integral_constant<
        bool, std::is_convertible<T4 const &,
                                  std::basic_string_view<T1, T2>>::value &&
                  std::integral_constant<
                      bool, !std::is_convertible<
                                T4 const *, const std::basic_string<T1, T2, T3>
                                                *>::value>::value &&
                  std::integral_constant<
                      bool, !std::is_convertible<
                                T4 const &, const T1 *>::value>::value>::value,
    std::basic_string<T1, T2, T3> &>
f224(std::basic_string<T1, T2, T3> &a0, const T4 &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.template assign<T4>(a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
std::enable_if_t<
    std::integral_constant<
        bool, std::is_convertible<T4 const &,
                                  std::basic_string_view<T1, T2>>::value &&
                  std::integral_constant<
                      bool, !std::is_convertible<
                                T4 const *, const std::basic_string<T1, T2, T3>
                                                *>::value>::value &&
                  std::integral_constant<
                      bool, !std::is_convertible<
                                T4 const &, const T1 *>::value>::value>::value,
    std::basic_string<T1, T2, T3> &>
f225(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1, const T4 &a2) {
  return a0.template insert<T4>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::enable_if_t<
    std::integral_constant<
        bool, std::is_convertible<T4 const &,
                                  std::basic_string_view<T1, T2>>::value &&
                  std::integral_constant<
                      bool, !std::is_convertible<
                                T4 const *, const std::basic_string<T1, T2, T3>
                                                *>::value>::value &&
                  std::integral_constant<
                      bool, !std::is_convertible<
                                T4 const &, const T1 *>::value>::value>::value,
    std::basic_string<T1, T2, T3> &>
f226(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1, const T4 &a2,
     typename std::basic_string<T1, T2, T3>::size_type a3,
     typename std::basic_string<T1, T2, T3>::size_type a4) {
  return a0.template insert<T4>(a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4>
std::enable_if_t<
    std::integral_constant<
        bool, std::is_convertible<T4 const &,
                                  std::basic_string_view<T1, T2>>::value &&
                  std::integral_constant<
                      bool, !std::is_convertible<
                                T4 const *, const std::basic_string<T1, T2, T3>
                                                *>::value>::value &&
                  std::integral_constant<
                      bool, !std::is_convertible<
                                T4 const &, const T1 *>::value>::value>::value,
    std::basic_string<T1, T2, T3> &>
f227(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2, const T4 &a3) {
  return a0.template replace<T4>(a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
std::enable_if_t<
    std::integral_constant<
        bool, std::is_convertible<T4 const &,
                                  std::basic_string_view<T1, T2>>::value &&
                  std::integral_constant<
                      bool, !std::is_convertible<
                                T4 const *, const std::basic_string<T1, T2, T3>
                                                *>::value>::value &&
                  std::integral_constant<
                      bool, !std::is_convertible<
                                T4 const &, const T1 *>::value>::value>::value,
    std::basic_string<T1, T2, T3> &>
f228(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2, const T4 &a3,
     typename std::basic_string<T1, T2, T3>::size_type a4,
     typename std::basic_string<T1, T2, T3>::size_type a5) {
  return a0.template replace<T4>(a1, a2, a3, a4, a5);
}

template <typename T1, typename T2, typename T3, typename T4>
std::enable_if_t<
    std::integral_constant<
        bool, std::is_convertible<T4 const &,
                                  std::basic_string_view<T1, T2>>::value &&
                  std::integral_constant<
                      bool, !std::is_convertible<
                                T4 const *, const std::basic_string<T1, T2, T3>
                                                *>::value>::value &&
                  std::integral_constant<
                      bool, !std::is_convertible<
                                T4 const &, const T1 *>::value>::value>::value,
    std::basic_string<T1, T2, T3> &>
f229(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1,
     typename std::basic_string<T1, T2, T3>::const_iterator a2, const T4 &a3) {
  return a0.template replace<T4>(a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
std::enable_if_t<
    std::integral_constant<
        bool, std::is_convertible<T4 const &,
                                  std::basic_string_view<T1, T2>>::value &&
                  std::integral_constant<
                      bool, !std::is_convertible<
                                T4 const *, const std::basic_string<T1, T2, T3>
                                                *>::value>::value &&
                  std::integral_constant<
                      bool, !std::is_convertible<
                                T4 const &, const T1 *>::value>::value>::value,
    int>
f230(const std::basic_string<T1, T2, T3> &a0, const T4 &a1) {
  return a0.template compare<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::enable_if_t<
    std::integral_constant<
        bool, std::is_convertible<T4 const &,
                                  std::basic_string_view<T1, T2>>::value &&
                  std::integral_constant<
                      bool, !std::is_convertible<
                                T4 const *, const std::basic_string<T1, T2, T3>
                                                *>::value>::value &&
                  std::integral_constant<
                      bool, !std::is_convertible<
                                T4 const &, const T1 *>::value>::value>::value,
    int>
f231(const std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2, const T4 &a3) {
  return a0.template compare<T4>(a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
std::enable_if_t<
    std::integral_constant<
        bool, std::is_convertible<T4 const &,
                                  std::basic_string_view<T1, T2>>::value &&
                  std::integral_constant<
                      bool, !std::is_convertible<
                                T4 const *, const std::basic_string<T1, T2, T3>
                                                *>::value>::value &&
                  std::integral_constant<
                      bool, !std::is_convertible<
                                T4 const &, const T1 *>::value>::value>::value,
    int>
f232(const std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2, const T4 &a3,
     typename std::basic_string<T1, T2, T3>::size_type a4,
     typename std::basic_string<T1, T2, T3>::size_type a5) {
  return a0.template compare<T4>(a1, a2, a3, a4, a5);
}
