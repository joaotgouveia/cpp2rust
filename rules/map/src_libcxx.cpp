#include <initializer_list>
#include <map>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
bool f103(const std::map<T1, T2, T3, T4> &a0,
          const std::map<T1, T2, T3, T4> &a1) {
  return std::operator== <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f104(const std::multimap<T1, T2, T3, T4> &a0,
          const std::multimap<T1, T2, T3, T4> &a1) {
  return std::operator== <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
void f105(std::map<T1, T2, T3, T4> &a0, std::map<T1, T2, T3, T4> &a1) {
  return std::swap<T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
void f106(std::multimap<T1, T2, T3, T4> &a0,
          std::multimap<T1, T2, T3, T4> &a1) {
  return std::swap<T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::map<T1, T2, T3, T4> f107(const T3 &a0) {
  return std::map<T1, T2, T3, T4>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::map<T1, T2, T3, T4> f108(std::initializer_list<std::pair<const T1, T2>> a0,
                              const T3 &a1) {
  return std::map<T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::map<T1, T2, T3, T4> f109(T5 a0, T5 a1, const T3 &a2) {
  return std::map<T1, T2, T3, T4>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          std::__enable_if_t<std::__is_transparently_comparable_v<
                                 T3, T1, std::__remove_cvref_t<T5>>,
                             int>
              T6 = 0>
T2 &f110(std::map<T1, T2, T3, T4> &a0, T5 &&a1) {
  return a0.template at<T5>(std::forward<T5>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          std::__enable_if_t<std::__is_transparently_comparable_v<
                                 T3, T1, std::__remove_cvref_t<T5>>,
                             int>
              T6 = 0>
const T2 &f111(const std::map<T1, T2, T3, T4> &a0, T5 &&a1) {
  return a0.template at<T5>(std::forward<T5>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          std::__enable_if_t<
              std::is_constructible<std::pair<const T1, T2>, T5>::value, int>
              T6 = 0>
std::pair<typename std::map<T1, T2, T3, T4>::iterator, bool>
f112(std::map<T1, T2, T3, T4> &a0, T5 &&a1) {
  return a0.template insert<T5>(std::forward<T5>(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          std::__enable_if_t<
              std::is_constructible<std::pair<const T1, T2>, T5>::value, int>
              T6 = 0>
typename std::map<T1, T2, T3, T4>::iterator
f113(std::map<T1, T2, T3, T4> &a0,
     typename std::map<T1, T2, T3, T4>::const_iterator a1, T5 &&a2) {
  return a0.template insert<T5>(a1, std::forward<T5>(a2));
}
