#include <initializer_list>
#include <map>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
bool f102(const std::map<T1, T2, T3, T4> &a0,
          const std::map<T1, T2, T3, T4> &a1) {
  return std::operator!= <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f103(const std::multimap<T1, T2, T3, T4> &a0,
          const std::multimap<T1, T2, T3, T4> &a1) {
  return std::operator!= <T1, T2, T3, T4>(a0, a1);
}

template <
    typename T1, typename T2, typename T3, typename T4, typename T5,
    std::__enable_if_t<
        std::__is_transparently_comparable_v<T3, T1, std::__remove_cvref_t<T5>>,
        int>
        T6 = 0,
    typename std::enable_if<
        (!(std::is_same<
            T5, const typename std::map<T1, T2, T3, T4>::key_type &>::value)),
        int>::type T7 = 0>
T2 &f104(std::map<T1, T2, T3, T4> &a0, T5 &&a1) {
  return a0.template at<T5>(std::forward<T5>(a1));
}

template <
    typename T1, typename T2, typename T3, typename T4, typename T5,
    std::__enable_if_t<
        std::__is_transparently_comparable_v<T3, T1, std::__remove_cvref_t<T5>>,
        int>
        T6 = 0,
    typename std::enable_if<
        (!(std::is_same<
            T5, const typename std::map<T1, T2, T3, T4>::key_type &>::value)),
        int>::type T7 = 0>
const T2 &f105(const std::map<T1, T2, T3, T4> &a0, T5 &&a1) {
  return a0.template at<T5>(std::forward<T5>(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
bool f106(const std::map<T1, T2, T3, T4> &a0,
          const std::map<T1, T2, T3, T4> &a1) {
  return std::operator< <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f107(const std::multimap<T1, T2, T3, T4> &a0,
          const std::multimap<T1, T2, T3, T4> &a1) {
  return std::operator< <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f108(const std::map<T1, T2, T3, T4> &a0,
          const std::map<T1, T2, T3, T4> &a1) {
  return std::operator<= <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f109(const std::multimap<T1, T2, T3, T4> &a0,
          const std::multimap<T1, T2, T3, T4> &a1) {
  return std::operator<= <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f110(const std::map<T1, T2, T3, T4> &a0,
          const std::map<T1, T2, T3, T4> &a1) {
  return std::operator><T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f111(const std::multimap<T1, T2, T3, T4> &a0,
          const std::multimap<T1, T2, T3, T4> &a1) {
  return std::operator><T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f112(const std::map<T1, T2, T3, T4> &a0,
          const std::map<T1, T2, T3, T4> &a1) {
  return std::operator>= <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f113(const std::multimap<T1, T2, T3, T4> &a0,
          const std::multimap<T1, T2, T3, T4> &a1) {
  return std::operator>= <T1, T2, T3, T4>(a0, a1);
}
