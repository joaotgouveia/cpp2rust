#include <algorithm>
#include <initializer_list>
#include <string>
#include <utility>

template <typename T1, typename T2, typename T3>
std::basic_string_view<T1, T2> f223(const std::basic_string<T1, T2, T3> &a0) {
  return static_cast<std::basic_string_view<T1, T2>>(a0);
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::_RequireAllocator<T3>>
std::basic_string<T1, T2, T3> f224(const T1 *a0, const T3 &a1) {
  return std::basic_string<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3,
          typename T4 = std::_RequireAllocator<T3>>
std::basic_string<T1, T2, T3>
f225(typename std::basic_string<T1, T2, T3>::size_type a0, T1 a1,
     const T3 &a2) {
  return std::basic_string<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename T5 = std::enable_if_t<std::is_convertible_v<
              const T4 &, std::basic_string_view<T1, T2>>>>
std::basic_string<T1, T2, T3>
f226(const T4 &a0, typename std::basic_string<T1, T2, T3>::size_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2, const T3 &a3) {
  return std::basic_string<T1, T2, T3>(a0, a1, a2, a3);
}

template <
    typename T1, typename T2, typename T3, typename T4,
    typename T5 = std::enable_if_t<
        std::__and_<
            std::is_convertible<T4 const &, std::basic_string_view<T1, T2>>,
            std::__not_<std::is_convertible<
                T4 const *, const std::basic_string<T1, T2, T3> *>>,
            std::__not_<std::is_convertible<T4 const &, const T1 *>>>::value,
        void>>
std::basic_string<T1, T2, T3> f227(const T4 &a0, const T3 &a1) {
  return std::basic_string<T1, T2, T3>(a0, a1);
}
