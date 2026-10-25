#include <algorithm>
#include <ios>
#include <regex>
#include <string>
#include <utility>

template <typename T1>
bool f11(const std::regex_traits<T1> &a0, T1 a1,
         typename std::regex_traits<T1>::char_class_type a2) {
  return a0.isctype(
      std::move(a1),
      std::forward<typename std::regex_traits<T1>::char_class_type>(a2));
}

template <typename T1, typename T2>
typename std::regex_traits<T1>::char_class_type
f12(const std::regex_traits<T1> &a0, T2 a1, T2 a2, bool a3) {
  return a0.template lookup_classname<T2>(std::move(a1), std::move(a2), a3);
}

template <typename T1> using t6 = typename std::regex_traits<T1>::_RegexMask;

template <typename T1>
typename std::regex_traits<T1>::_RegexMask f13(std::ctype_base::mask a0,
                                               unsigned char a1) {
  return typename std::regex_traits<T1>::_RegexMask(a0, a1);
}

template <typename T1>
typename std::regex_traits<T1>::_RegexMask
f14(const typename std::regex_traits<T1>::_RegexMask &a0,
    typename std::regex_traits<T1>::_RegexMask a1) {
  return a0.operator&(
      std::forward<typename std::regex_traits<T1>::_RegexMask>(a1));
}

template <typename T1>
typename std::regex_traits<T1>::_RegexMask
f15(const typename std::regex_traits<T1>::_RegexMask &a0,
    typename std::regex_traits<T1>::_RegexMask a1) {
  return a0.operator|(
      std::forward<typename std::regex_traits<T1>::_RegexMask>(a1));
}

template <typename T1>
typename std::regex_traits<T1>::_RegexMask
f16(const typename std::regex_traits<T1>::_RegexMask &a0,
    typename std::regex_traits<T1>::_RegexMask a1) {
  return a0.operator^(
      std::forward<typename std::regex_traits<T1>::_RegexMask>(a1));
}

template <typename T1>
typename std::regex_traits<T1>::_RegexMask
f17(const typename std::regex_traits<T1>::_RegexMask &a0) {
  return a0.operator~();
}

template <typename T1>
typename std::regex_traits<T1>::_RegexMask &
f18(typename std::regex_traits<T1>::_RegexMask &a0,
    typename std::regex_traits<T1>::_RegexMask a1) {
  return a0.operator&=(
      std::forward<typename std::regex_traits<T1>::_RegexMask>(a1));
}

template <typename T1>
typename std::regex_traits<T1>::_RegexMask &
f19(typename std::regex_traits<T1>::_RegexMask &a0,
    typename std::regex_traits<T1>::_RegexMask a1) {
  return a0.operator|=(
      std::forward<typename std::regex_traits<T1>::_RegexMask>(a1));
}

template <typename T1>
typename std::regex_traits<T1>::_RegexMask &
f20(typename std::regex_traits<T1>::_RegexMask &a0,
    typename std::regex_traits<T1>::_RegexMask a1) {
  return a0.operator^=(
      std::forward<typename std::regex_traits<T1>::_RegexMask>(a1));
}

template <typename T1>
bool f21(const typename std::regex_traits<T1>::_RegexMask &a0,
         typename std::regex_traits<T1>::_RegexMask a1) {
  return a0.operator==(
      std::forward<typename std::regex_traits<T1>::_RegexMask>(a1));
}
