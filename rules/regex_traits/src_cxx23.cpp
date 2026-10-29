#include <algorithm>
#include <ios>
#include <regex>
#include <utility>

template <typename T1> using t1 = std::regex_traits<T1>;

template <typename T1> using t2 = typename std::regex_traits<T1>::char_type;

template <typename T1> using t3 = typename std::regex_traits<T1>::string_type;

template <typename T1> using t4 = typename std::regex_traits<T1>::locale_type;

template <typename T1>
using t5 = typename std::regex_traits<T1>::char_class_type;

template <typename T1> std::regex_traits<T1> f1() {
  return std::regex_traits<T1>();
}

template <typename T1> std::size_t f2(const T1 *a0) {
  return std::regex_traits<T1>::length(a0);
}

template <typename T1> T1 f3(const std::regex_traits<T1> &a0, T1 a1) {
  return a0.translate(std::move(a1));
}

template <typename T1> T1 f4(const std::regex_traits<T1> &a0, T1 a1) {
  return a0.translate_nocase(std::move(a1));
}

template <typename T1>
bool f5(const std::regex_traits<T1> &a0, T1 a1,
        typename std::regex_traits<T1>::char_class_type a2) {
  return a0.isctype(
      std::move(a1),
      std::forward<typename std::regex_traits<T1>::char_class_type>(a2));
}

template <typename T1> int f6(const std::regex_traits<T1> &a0, T1 a1, int a2) {
  return a0.value(std::move(a1), a2);
}

template <typename T1>
std::locale f7(std::regex_traits<T1> &a0, std::locale a1) {
  return a0.imbue(std::move(a1));
}

template <typename T1> std::locale f8(const std::regex_traits<T1> &a0) {
  return a0.getloc();
}

template <typename T1, typename T2>
std::basic_string<T1> f9(const std::regex_traits<T1> &a0, T2 a1, T2 a2) {
  return a0.template transform<T2>(std::move(a1), std::move(a2));
}

template <typename T1, typename T2>
std::basic_string<T1> f10(const std::regex_traits<T1> &a0, T2 a1, T2 a2) {
  return a0.template transform_primary<T2>(std::move(a1), std::move(a2));
}

template <typename T1, typename T2>
std::basic_string<T1> f11(const std::regex_traits<T1> &a0, T2 a1, T2 a2) {
  return a0.template lookup_collatename<T2>(std::move(a1), std::move(a2));
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
