#include <regex>
#include <string>
#include <utility>

template <typename T1, typename T2> using t1 = std::match_results<T1, T2>;

template <typename T1, typename T2>
using t2 = typename std::match_results<T1, T2>::value_type;

template <typename T1, typename T2>
using t3 = typename std::match_results<T1, T2>::const_reference;

template <typename T1, typename T2>
using t4 = typename std::match_results<T1, T2>::reference;

template <typename T1, typename T2>
using t5 = typename std::match_results<T1, T2>::const_iterator;

template <typename T1, typename T2>
using t6 = typename std::match_results<T1, T2>::iterator;

template <typename T1, typename T2>
using t7 = typename std::match_results<T1, T2>::difference_type;

template <typename T1, typename T2>
using t8 = typename std::match_results<T1, T2>::size_type;

template <typename T1, typename T2>
using t9 = typename std::match_results<T1, T2>::allocator_type;

template <typename T1, typename T2>
using t10 = typename std::match_results<T1, T2>::char_type;

template <typename T1, typename T2>
using t11 = typename std::match_results<T1, T2>::string_type;

template <typename T1, typename T2> std::match_results<T1, T2> f1() {
  return std::match_results<T1, T2>();
}

template <typename T1, typename T2>
std::match_results<T1, T2> f2(const T2 &a0) {
  return std::match_results<T1, T2>(a0);
}

template <typename T1, typename T2>
bool f3(const std::match_results<T1, T2> &a0) {
  return a0.ready();
}

template <typename T1, typename T2>
typename std::match_results<T1, T2>::size_type
f4(const std::match_results<T1, T2> &a0) {
  return a0.size();
}

template <typename T1, typename T2>
typename std::match_results<T1, T2>::size_type
f5(const std::match_results<T1, T2> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2>
bool f6(const std::match_results<T1, T2> &a0) {
  return a0.empty();
}

template <typename T1, typename T2>
typename std::match_results<T1, T2>::difference_type
f7(const std::match_results<T1, T2> &a0,
   typename std::match_results<T1, T2>::size_type a1) {
  return a0.length(std::move(a1));
}

template <typename T1, typename T2>
typename std::match_results<T1, T2>::difference_type
f8(const std::match_results<T1, T2> &a0,
   typename std::match_results<T1, T2>::size_type a1) {
  return a0.position(std::move(a1));
}

template <typename T1, typename T2>
typename std::match_results<T1, T2>::string_type
f9(const std::match_results<T1, T2> &a0,
   typename std::match_results<T1, T2>::size_type a1) {
  return a0.str(std::move(a1));
}

template <typename T1, typename T2>
const std::sub_match<T1> &
f10(const std::match_results<T1, T2> &a0,
    typename std::match_results<T1, T2>::size_type a1) {
  return a0.operator[](std::move(a1));
}

template <typename T1, typename T2>
const std::sub_match<T1> &f11(const std::match_results<T1, T2> &a0) {
  return a0.prefix();
}

template <typename T1, typename T2>
const std::sub_match<T1> &f12(const std::match_results<T1, T2> &a0) {
  return a0.suffix();
}

template <typename T1, typename T2>
typename std::match_results<T1, T2>::const_iterator
f13(const std::match_results<T1, T2> &a0) {
  return a0.begin();
}

template <typename T1, typename T2>
typename std::match_results<T1, T2>::const_iterator
f14(const std::match_results<T1, T2> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2>
typename std::match_results<T1, T2>::const_iterator
f15(const std::match_results<T1, T2> &a0) {
  return a0.end();
}

template <typename T1, typename T2>
typename std::match_results<T1, T2>::const_iterator
f16(const std::match_results<T1, T2> &a0) {
  return a0.cend();
}

template <typename T1, typename T2>
typename std::match_results<T1, T2>::string_type
f17(const std::match_results<T1, T2> &a0,
    const typename std::match_results<T1, T2>::char_type *a1,
    std::regex_constants::match_flag_type a2) {
  return a0.format(a1, a2);
}

template <typename T1, typename T2>
T2 f18(const std::match_results<T1, T2> &a0) {
  return a0.get_allocator();
}

template <typename T1, typename T2>
void f19(std::match_results<T1, T2> &a0, std::match_results<T1, T2> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2, typename T3>
T3 f20(const std::match_results<T1, T2> &a0, T3 a1,
       const typename std::match_results<T1, T2>::char_type *a2,
       const typename std::match_results<T1, T2>::char_type *a3,
       std::regex_constants::match_flag_type a4) {
  return a0.template format<T3>(std::move(a1), a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
T3 f21(const std::match_results<T1, T2> &a0, T3 a1,
       const std::basic_string<typename std::match_results<T1, T2>::char_type,
                               T4, T5> &a2,
       std::regex_constants::match_flag_type a3) {
  return a0.template format<T3, T4, T5>(std::move(a1), a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<typename std::match_results<T1, T2>::char_type, T3, T4>
f22(const std::match_results<T1, T2> &a0,
    const std::basic_string<typename std::match_results<T1, T2>::char_type, T3,
                            T4> &a1,
    std::regex_constants::match_flag_type a2) {
  return a0.template format<T3, T4>(a1, a2);
}
