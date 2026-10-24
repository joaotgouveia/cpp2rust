#include <algorithm>
#include <initializer_list>
#include <string>
#include <utility>

template <typename T1, typename T2, typename T3>
std::basic_string<T1, T2, T3> f125() {
  return std::basic_string<T1, T2, T3>();
}

template <typename T1, typename T2, typename T3>
typename std::basic_string<T1, T2, T3>::allocator_type
f126(const std::basic_string<T1, T2, T3> &a0) {
  return a0.get_allocator();
}

template <typename T1, typename T2, typename T3, typename T4,
          typename T5 = std::_RequireInputIter<T4>,
          typename std::enable_if<
              (!(std::is_same<T4, T1 *>::value) &&
               !(std::is_same<T4, const T1 *>::value) &&
               !(std::is_same<T4, typename std::basic_string<
                                      T1, T2, T3>::iterator>::value) &&
               !(std::is_same<T4, typename std::basic_string<
                                      T1, T2, T3>::const_iterator>::value)),
              int>::type T6 = 0>
std::basic_string<T1, T2, T3> &
f127(std::basic_string<T1, T2, T3> &a0,
     typename std::basic_string<T1, T2, T3>::const_iterator a1,
     typename std::basic_string<T1, T2, T3>::const_iterator a2, T4 a3, T4 a4) {
  return a0.template replace<T4>(std::move(a1), std::move(a2), std::move(a3),
                                 std::move(a4));
}

template <typename T1, typename T2, typename T3, typename T4,
          typename T5 = std::__enable_if_t<std::is_convertible<
              T4, typename std::basic_string<T1, T2, T3>::pointer>::value>>
typename std::basic_string<T1, T2, T3>::iterator f128(
    const __gnu_cxx::__normal_iterator<T4, std::basic_string<T1, T2, T3>> &a0) {
  return typename std::basic_string<T1, T2, T3>::iterator(a0);
}

template <
    typename T1, typename T2, typename T3, typename T4,
    typename T5 = std::__enable_if_t<std::is_convertible<
        T4, typename std::basic_string<T1, T2, T3>::const_pointer>::value>>
typename std::basic_string<T1, T2, T3>::const_iterator f129(
    const __gnu_cxx::__normal_iterator<T4, std::basic_string<T1, T2, T3>> &a0) {
  return typename std::basic_string<T1, T2, T3>::const_iterator(a0);
}

template <typename T1, typename T2, typename T3>
bool f130(const std::basic_string<T1, T2, T3> &a0, const T1 *a1) {
  return std::operator!= <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f131(const std::basic_string<T1, T2, T3> &a0,
          const std::basic_string<T1, T2, T3> &a1) {
  return std::operator!= <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f132(const T1 *a0, const std::basic_string<T1, T2, T3> &a1) {
  return std::operator!= <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f133(const std::basic_string<T1, T2, T3> &a0, const T1 *a1) {
  return std::operator< <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f134(const std::basic_string<T1, T2, T3> &a0,
          const std::basic_string<T1, T2, T3> &a1) {
  return std::operator< <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f135(const T1 *a0, const std::basic_string<T1, T2, T3> &a1) {
  return std::operator< <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f136(const std::basic_string<T1, T2, T3> &a0, const T1 *a1) {
  return std::operator<= <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f137(const std::basic_string<T1, T2, T3> &a0,
          const std::basic_string<T1, T2, T3> &a1) {
  return std::operator<= <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f138(const T1 *a0, const std::basic_string<T1, T2, T3> &a1) {
  return std::operator<= <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f139(const T1 *a0, const std::basic_string<T1, T2, T3> &a1) {
  return std::operator== <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f140(const std::basic_string<T1, T2, T3> &a0, const T1 *a1) {
  return std::operator><T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f141(const std::basic_string<T1, T2, T3> &a0,
          const std::basic_string<T1, T2, T3> &a1) {
  return std::operator><T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f142(const T1 *a0, const std::basic_string<T1, T2, T3> &a1) {
  return std::operator><T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f143(const std::basic_string<T1, T2, T3> &a0, const T1 *a1) {
  return std::operator>= <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f144(const std::basic_string<T1, T2, T3> &a0,
          const std::basic_string<T1, T2, T3> &a1) {
  return std::operator>= <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f145(const T1 *a0, const std::basic_string<T1, T2, T3> &a1) {
  return std::operator>= <T1, T2, T3>(a0, a1);
}
