#include <ostream>
#include <regex>
#include <string>
#include <utility>

template <typename T1>
bool f1(const std::sub_match<T1> &a0,
        const typename std::iterator_traits<T1>::value_type &a1) {
  return std::operator!= <T1>(a0, a1);
}

template <typename T1>
bool f2(const std::sub_match<T1> &a0,
        const typename std::iterator_traits<T1>::value_type *a1) {
  return std::operator!= <T1>(a0, a1);
}

template <typename T1>
bool f3(const std::sub_match<T1> &a0, const std::sub_match<T1> &a1) {
  return std::operator!= <T1>(a0, a1);
}

template <typename T1>
bool f4(const typename std::iterator_traits<T1>::value_type &a0,
        const std::sub_match<T1> &a1) {
  return std::operator!= <T1>(a0, a1);
}

template <typename T1>
bool f5(const typename std::iterator_traits<T1>::value_type *a0,
        const std::sub_match<T1> &a1) {
  return std::operator!= <T1>(a0, a1);
}

template <typename T1, typename T2>
bool f6(const std::match_results<T1, T2> &a0,
        const std::match_results<T1, T2> &a1) {
  return std::operator!= <T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f7(const std::sub_match<T1> &a0,
        const std::basic_string<typename std::iterator_traits<T1>::value_type,
                                T2, T3> &a1) {
  return std::operator!= <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f8(const std::basic_string<typename std::iterator_traits<T1>::value_type,
                                T2, T3> &a0,
        const std::sub_match<T1> &a1) {
  return std::operator!= <T1, T2, T3>(a0, a1);
}

template <typename T1>
bool f9(const std::sub_match<T1> &a0,
        const typename std::iterator_traits<T1>::value_type &a1) {
  return std::operator< <T1>(a0, a1);
}

template <typename T1>
bool f10(const std::sub_match<T1> &a0,
         const typename std::iterator_traits<T1>::value_type *a1) {
  return std::operator< <T1>(a0, a1);
}

template <typename T1>
bool f11(const std::sub_match<T1> &a0, const std::sub_match<T1> &a1) {
  return std::operator< <T1>(a0, a1);
}

template <typename T1>
bool f12(const typename std::iterator_traits<T1>::value_type &a0,
         const std::sub_match<T1> &a1) {
  return std::operator< <T1>(a0, a1);
}

template <typename T1>
bool f13(const typename std::iterator_traits<T1>::value_type *a0,
         const std::sub_match<T1> &a1) {
  return std::operator< <T1>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f14(const std::sub_match<T1> &a0,
         const std::basic_string<typename std::iterator_traits<T1>::value_type,
                                 T2, T3> &a1) {
  return std::operator< <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f15(const std::basic_string<typename std::iterator_traits<T1>::value_type,
                                 T2, T3> &a0,
         const std::sub_match<T1> &a1) {
  return std::operator< <T1, T2, T3>(a0, a1);
}

template <typename T1>
bool f16(const std::sub_match<T1> &a0,
         const typename std::iterator_traits<T1>::value_type &a1) {
  return std::operator<= <T1>(a0, a1);
}

template <typename T1>
bool f17(const std::sub_match<T1> &a0,
         const typename std::iterator_traits<T1>::value_type *a1) {
  return std::operator<= <T1>(a0, a1);
}

template <typename T1>
bool f18(const std::sub_match<T1> &a0, const std::sub_match<T1> &a1) {
  return std::operator<= <T1>(a0, a1);
}

template <typename T1>
bool f19(const typename std::iterator_traits<T1>::value_type &a0,
         const std::sub_match<T1> &a1) {
  return std::operator<= <T1>(a0, a1);
}

template <typename T1>
bool f20(const typename std::iterator_traits<T1>::value_type *a0,
         const std::sub_match<T1> &a1) {
  return std::operator<= <T1>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f21(const std::sub_match<T1> &a0,
         const std::basic_string<typename std::iterator_traits<T1>::value_type,
                                 T2, T3> &a1) {
  return std::operator<= <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f22(const std::basic_string<typename std::iterator_traits<T1>::value_type,
                                 T2, T3> &a0,
         const std::sub_match<T1> &a1) {
  return std::operator<= <T1, T2, T3>(a0, a1);
}

template <typename T1>
bool f23(const typename std::iterator_traits<T1>::value_type &a0,
         const std::sub_match<T1> &a1) {
  return std::operator== <T1>(a0, a1);
}

template <typename T1>
bool f24(const typename std::iterator_traits<T1>::value_type *a0,
         const std::sub_match<T1> &a1) {
  return std::operator== <T1>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f25(const std::basic_string<typename std::iterator_traits<T1>::value_type,
                                 T2, T3> &a0,
         const std::sub_match<T1> &a1) {
  return std::operator== <T1, T2, T3>(a0, a1);
}

template <typename T1>
bool f26(const std::sub_match<T1> &a0,
         const typename std::iterator_traits<T1>::value_type &a1) {
  return std::operator><T1>(a0, a1);
}

template <typename T1>
bool f27(const std::sub_match<T1> &a0,
         const typename std::iterator_traits<T1>::value_type *a1) {
  return std::operator><T1>(a0, a1);
}

template <typename T1>
bool f28(const std::sub_match<T1> &a0, const std::sub_match<T1> &a1) {
  return std::operator><T1>(a0, a1);
}

template <typename T1>
bool f29(const typename std::iterator_traits<T1>::value_type &a0,
         const std::sub_match<T1> &a1) {
  return std::operator><T1>(a0, a1);
}

template <typename T1>
bool f30(const typename std::iterator_traits<T1>::value_type *a0,
         const std::sub_match<T1> &a1) {
  return std::operator><T1>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f31(const std::sub_match<T1> &a0,
         const std::basic_string<typename std::iterator_traits<T1>::value_type,
                                 T2, T3> &a1) {
  return std::operator><T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f32(const std::basic_string<typename std::iterator_traits<T1>::value_type,
                                 T2, T3> &a0,
         const std::sub_match<T1> &a1) {
  return std::operator><T1, T2, T3>(a0, a1);
}

template <typename T1>
bool f33(const std::sub_match<T1> &a0,
         const typename std::iterator_traits<T1>::value_type &a1) {
  return std::operator>= <T1>(a0, a1);
}

template <typename T1>
bool f34(const std::sub_match<T1> &a0,
         const typename std::iterator_traits<T1>::value_type *a1) {
  return std::operator>= <T1>(a0, a1);
}

template <typename T1>
bool f35(const std::sub_match<T1> &a0, const std::sub_match<T1> &a1) {
  return std::operator>= <T1>(a0, a1);
}

template <typename T1>
bool f36(const typename std::iterator_traits<T1>::value_type &a0,
         const std::sub_match<T1> &a1) {
  return std::operator>= <T1>(a0, a1);
}

template <typename T1>
bool f37(const typename std::iterator_traits<T1>::value_type *a0,
         const std::sub_match<T1> &a1) {
  return std::operator>= <T1>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f38(const std::sub_match<T1> &a0,
         const std::basic_string<typename std::iterator_traits<T1>::value_type,
                                 T2, T3> &a1) {
  return std::operator>= <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f39(const std::basic_string<typename std::iterator_traits<T1>::value_type,
                                 T2, T3> &a0,
         const std::sub_match<T1> &a1) {
  return std::operator>= <T1, T2, T3>(a0, a1);
}
