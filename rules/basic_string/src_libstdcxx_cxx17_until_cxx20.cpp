#include <algorithm>
#include <initializer_list>
#include <string>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4,
          typename T5 = std::enable_if_t<std::is_convertible_v<
              const T4 &, std::basic_string_view<T1, T2>>>,
          typename std::enable_if<
              (!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value)),
              int>::type T6 = 0>
std::basic_string<T1, T2, T3>
f225(const T4 &a0, typename std::basic_string<T1, T2, T3>::size_type a1,
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
        void>,
    typename std::enable_if<
        (!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value)), int>::type
        T6 = 0>
std::basic_string<T1, T2, T3> f226(const T4 &a0, const T3 &a1) {
  return std::basic_string<T1, T2, T3>(a0, a1);
}
