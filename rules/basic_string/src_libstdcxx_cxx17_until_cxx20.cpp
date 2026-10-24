#include <algorithm>
#include <initializer_list>
#include <string>
#include <type_traits>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4,
          typename T5 = std::enable_if_t<std::is_convertible_v<
              const T4 &, std::basic_string_view<T1, T2>>>,
          typename std::enable_if<
              (!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value)),
              int>::type T6 = 0>
std::basic_string<T1, T2, T3>
f220(const T4 &a0, typename std::basic_string<T1, T2, T3>::size_type a1,
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
std::basic_string<T1, T2, T3> f221(const T4 &a0, const T3 &a1) {
  return std::basic_string<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename std::enable_if<
              (!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value)),
              int>::type T5 = 0>
std::enable_if_t<
    std::__and_<
        std::is_convertible<T4 const &, std::basic_string_view<T1, T2>>,
        std::__not_<std::is_convertible<T4 const *,
                                        const std::basic_string<T1, T2, T3> *>>,
        std::__not_<std::is_convertible<T4 const &, const T1 *>>>::value,
    std::basic_string<T1, T2, T3> &>
f222(std::basic_string<T1, T2, T3> &a0, const T4 &a1) {
  return a0.template operator= <T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename std::enable_if<
              (!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value)),
              int>::type T5 = 0>
std::enable_if_t<
    std::__and_<
        std::is_convertible<T4 const &, std::basic_string_view<T1, T2>>,
        std::__not_<std::is_convertible<T4 const *,
                                        const std::basic_string<T1, T2, T3> *>>,
        std::__not_<std::is_convertible<T4 const &, const T1 *>>>::value,
    std::basic_string<T1, T2, T3> &>
f223(std::basic_string<T1, T2, T3> &a0, const T4 &a1) {
  return a0.template operator+= <T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename std::enable_if<
              (!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value)),
              int>::type T5 = 0>
std::enable_if_t<
    std::__and_<
        std::is_convertible<T4 const &, std::basic_string_view<T1, T2>>,
        std::__not_<std::is_convertible<T4 const *,
                                        const std::basic_string<T1, T2, T3> *>>,
        std::__not_<std::is_convertible<T4 const &, const T1 *>>>::value,
    std::basic_string<T1, T2, T3> &>
f224(std::basic_string<T1, T2, T3> &a0, const T4 &a1) {
  return a0.template append<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename std::enable_if<
              (!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value)),
              int>::type T5 = 0>
std::enable_if_t<
    std::__and_<
        std::is_convertible<T4 const &, std::basic_string_view<T1, T2>>,
        std::__not_<std::is_convertible<T4 const *,
                                        const std::basic_string<T1, T2, T3> *>>,
        std::__not_<std::is_convertible<T4 const &, const T1 *>>>::value,
    std::basic_string<T1, T2, T3> &>
f225(std::basic_string<T1, T2, T3> &a0, const T4 &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.template append<T4>(a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename std::enable_if<
              (!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value)),
              int>::type T5 = 0>
std::enable_if_t<
    std::__and_<
        std::is_convertible<T4 const &, std::basic_string_view<T1, T2>>,
        std::__not_<std::is_convertible<T4 const *,
                                        const std::basic_string<T1, T2, T3> *>>,
        std::__not_<std::is_convertible<T4 const &, const T1 *>>>::value,
    std::basic_string<T1, T2, T3> &>
f226(std::basic_string<T1, T2, T3> &a0, const T4 &a1) {
  return a0.template assign<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename std::enable_if<
              (!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value)),
              int>::type T5 = 0>
std::enable_if_t<
    std::__and_<
        std::is_convertible<T4 const &, std::basic_string_view<T1, T2>>,
        std::__not_<std::is_convertible<T4 const *,
                                        const std::basic_string<T1, T2, T3> *>>,
        std::__not_<std::is_convertible<T4 const &, const T1 *>>>::value,
    std::basic_string<T1, T2, T3> &>
f227(std::basic_string<T1, T2, T3> &a0, const T4 &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2,
     typename std::basic_string<T1, T2, T3>::size_type a3) {
  return a0.template assign<T4>(a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename std::enable_if<
              (!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value)),
              int>::type T5 = 0>
std::enable_if_t<
    std::__and_<
        std::is_convertible<T4 const &, std::basic_string_view<T1, T2>>,
        std::__not_<std::is_convertible<T4 const *,
                                        const std::basic_string<T1, T2, T3> *>>,
        std::__not_<std::is_convertible<T4 const &, const T1 *>>>::value,
    std::basic_string<T1, T2, T3> &>
f228(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1, const T4 &a2) {
  return a0.template insert<T4>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename std::enable_if<
              (!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value)),
              int>::type T5 = 0>
std::enable_if_t<
    std::__and_<
        std::is_convertible<T4 const &, std::basic_string_view<T1, T2>>,
        std::__not_<std::is_convertible<T4 const *,
                                        const std::basic_string<T1, T2, T3> *>>,
        std::__not_<std::is_convertible<T4 const &, const T1 *>>>::value,
    std::basic_string<T1, T2, T3> &>
f229(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1, const T4 &a2,
     typename std::basic_string<T1, T2, T3>::size_type a3,
     typename std::basic_string<T1, T2, T3>::size_type a4) {
  return a0.template insert<T4>(a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename std::enable_if<
              (!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value)),
              int>::type T5 = 0>
std::enable_if_t<
    std::__and_<
        std::is_convertible<T4 const &, std::basic_string_view<T1, T2>>,
        std::__not_<std::is_convertible<T4 const *,
                                        const std::basic_string<T1, T2, T3> *>>,
        std::__not_<std::is_convertible<T4 const &, const T1 *>>>::value,
    std::basic_string<T1, T2, T3> &>
f230(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2, const T4 &a3) {
  return a0.template replace<T4>(a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename std::enable_if<
              (!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value)),
              int>::type T5 = 0>
std::enable_if_t<
    std::__and_<
        std::is_convertible<T4 const &, std::basic_string_view<T1, T2>>,
        std::__not_<std::is_convertible<T4 const *,
                                        const std::basic_string<T1, T2, T3> *>>,
        std::__not_<std::is_convertible<T4 const &, const T1 *>>>::value,
    std::basic_string<T1, T2, T3> &>
f231(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2, const T4 &a3,
     typename std::basic_string<T1, T2, T3>::size_type a4,
     typename std::basic_string<T1, T2, T3>::size_type a5) {
  return a0.template replace<T4>(a1, a2, a3, a4, a5);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename std::enable_if<
              (!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value)),
              int>::type T5 = 0>
std::enable_if_t<
    std::__and_<
        std::is_convertible<T4 const &, std::basic_string_view<T1, T2>>,
        std::__not_<std::is_convertible<T4 const *,
                                        const std::basic_string<T1, T2, T3> *>>,
        std::__not_<std::is_convertible<T4 const &, const T1 *>>>::value,
    std::basic_string<T1, T2, T3> &>
f232(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1,
     typename std::basic_string<T1, T2, T3>::const_iterator a2, const T4 &a3) {
  return a0.template replace<T4>(a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename std::enable_if<
              (!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value)),
              int>::type T5 = 0>
std::enable_if_t<
    std::__and_<
        std::is_convertible<T4 const &, std::basic_string_view<T1, T2>>,
        std::__not_<std::is_convertible<T4 const *,
                                        const std::basic_string<T1, T2, T3> *>>,
        std::__not_<std::is_convertible<T4 const &, const T1 *>>>::value,
    typename __gnu_cxx::__alloc_traits<typename __gnu_cxx::__alloc_traits<
        T3>::template rebind<T1>::other>::size_type>
f233(const std::basic_string<T1, T2, T3> &a0, const T4 &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.template find<T4>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename std::enable_if<
              (!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value)),
              int>::type T5 = 0>
std::enable_if_t<
    std::__and_<
        std::is_convertible<T4 const &, std::basic_string_view<T1, T2>>,
        std::__not_<std::is_convertible<T4 const *,
                                        const std::basic_string<T1, T2, T3> *>>,
        std::__not_<std::is_convertible<T4 const &, const T1 *>>>::value,
    typename __gnu_cxx::__alloc_traits<typename __gnu_cxx::__alloc_traits<
        T3>::template rebind<T1>::other>::size_type>
f234(const std::basic_string<T1, T2, T3> &a0, const T4 &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.template rfind<T4>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename std::enable_if<
              (!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value)),
              int>::type T5 = 0>
std::enable_if_t<
    std::__and_<
        std::is_convertible<T4 const &, std::basic_string_view<T1, T2>>,
        std::__not_<std::is_convertible<T4 const *,
                                        const std::basic_string<T1, T2, T3> *>>,
        std::__not_<std::is_convertible<T4 const &, const T1 *>>>::value,
    typename __gnu_cxx::__alloc_traits<typename __gnu_cxx::__alloc_traits<
        T3>::template rebind<T1>::other>::size_type>
f235(const std::basic_string<T1, T2, T3> &a0, const T4 &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.template find_first_of<T4>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename std::enable_if<
              (!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value)),
              int>::type T5 = 0>
std::enable_if_t<
    std::__and_<
        std::is_convertible<T4 const &, std::basic_string_view<T1, T2>>,
        std::__not_<std::is_convertible<T4 const *,
                                        const std::basic_string<T1, T2, T3> *>>,
        std::__not_<std::is_convertible<T4 const &, const T1 *>>>::value,
    typename __gnu_cxx::__alloc_traits<typename __gnu_cxx::__alloc_traits<
        T3>::template rebind<T1>::other>::size_type>
f236(const std::basic_string<T1, T2, T3> &a0, const T4 &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.template find_last_of<T4>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename std::enable_if<
              (!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value)),
              int>::type T5 = 0>
std::enable_if_t<
    std::__and_<
        std::is_convertible<T4 const &, std::basic_string_view<T1, T2>>,
        std::__not_<std::is_convertible<T4 const *,
                                        const std::basic_string<T1, T2, T3> *>>,
        std::__not_<std::is_convertible<T4 const &, const T1 *>>>::value,
    typename __gnu_cxx::__alloc_traits<typename __gnu_cxx::__alloc_traits<
        T3>::template rebind<T1>::other>::size_type>
f237(const std::basic_string<T1, T2, T3> &a0, const T4 &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.template find_first_not_of<T4>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename std::enable_if<
              (!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value)),
              int>::type T5 = 0>
std::enable_if_t<
    std::__and_<
        std::is_convertible<T4 const &, std::basic_string_view<T1, T2>>,
        std::__not_<std::is_convertible<T4 const *,
                                        const std::basic_string<T1, T2, T3> *>>,
        std::__not_<std::is_convertible<T4 const &, const T1 *>>>::value,
    typename __gnu_cxx::__alloc_traits<typename __gnu_cxx::__alloc_traits<
        T3>::template rebind<T1>::other>::size_type>
f238(const std::basic_string<T1, T2, T3> &a0, const T4 &a1,
     typename std::basic_string<T1, T2, T3>::size_type a2) {
  return a0.template find_last_not_of<T4>(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename std::enable_if<
              (!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value)),
              int>::type T5 = 0>
std::enable_if_t<
    std::__and_<
        std::is_convertible<T4 const &, std::basic_string_view<T1, T2>>,
        std::__not_<std::is_convertible<T4 const *,
                                        const std::basic_string<T1, T2, T3> *>>,
        std::__not_<std::is_convertible<T4 const &, const T1 *>>>::value,
    int>
f239(const std::basic_string<T1, T2, T3> &a0, const T4 &a1) {
  return a0.template compare<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename std::enable_if<
              (!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value)),
              int>::type T5 = 0>
std::enable_if_t<
    std::__and_<
        std::is_convertible<T4 const &, std::basic_string_view<T1, T2>>,
        std::__not_<std::is_convertible<T4 const *,
                                        const std::basic_string<T1, T2, T3> *>>,
        std::__not_<std::is_convertible<T4 const &, const T1 *>>>::value,
    int>
f240(const std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2, const T4 &a3) {
  return a0.template compare<T4>(a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4,
          typename std::enable_if<
              (!(std::is_same<T4, std::basic_string<T1, T2, T3>>::value)),
              int>::type T5 = 0>
std::enable_if_t<
    std::__and_<
        std::is_convertible<T4 const &, std::basic_string_view<T1, T2>>,
        std::__not_<std::is_convertible<T4 const *,
                                        const std::basic_string<T1, T2, T3> *>>,
        std::__not_<std::is_convertible<T4 const &, const T1 *>>>::value,
    int>
f241(const std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::size_type a1,
     typename std::basic_string<T1, T2, T3>::size_type a2, const T4 &a3,
     typename std::basic_string<T1, T2, T3>::size_type a4,
     typename std::basic_string<T1, T2, T3>::size_type a5) {
  return a0.template compare<T4>(a1, a2, a3, a4, a5);
}
