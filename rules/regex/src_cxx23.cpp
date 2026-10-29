#include <barrier>
#include <regex>
#include <utility>

template <typename T1, typename T2, typename T3>
std::basic_ostream<T1, T2> &f1(std::basic_ostream<T1, T2> &a0,
                               const std::sub_match<T3> &a1) {
  return std::operator<<(a0, a1);
}

template <typename T1>
auto f2(const std::sub_match<T1> &a0,
        const typename std::iterator_traits<T1>::value_type &a1) {
  return std::operator<=>(a0, a1);
}

template <typename T1>
auto f3(const std::sub_match<T1> &a0,
        const typename std::iterator_traits<T1>::value_type *a1) {
  return std::operator<=>(a0, a1);
}

template <typename T1>
auto f4(const std::sub_match<T1> &a0, const std::sub_match<T1> &a1) {
  return std::operator<=>(a0, a1);
}

template <typename T1, typename T2, typename T3>
auto f5(const std::sub_match<T1> &a0,
        const std::basic_string<typename std::iterator_traits<T1>::value_type,
                                T2, T3> &a1) {
  return std::operator<=>(a0, a1);
}

template <typename T1>
bool f6(const std::sub_match<T1> &a0,
        const typename std::iterator_traits<T1>::value_type &a1) {
  return std::operator==(a0, a1);
}

template <typename T1>
bool f7(const std::sub_match<T1> &a0,
        const typename std::iterator_traits<T1>::value_type *a1) {
  return std::operator==(a0, a1);
}

template <typename T1>
bool f8(const std::sub_match<T1> &a0, const std::sub_match<T1> &a1) {
  return std::operator==(a0, a1);
}

template <typename T1, typename T2>
bool f9(const std::match_results<T1, T2> &a0,
        const std::match_results<T1, T2> &a1) {
  return std::operator==(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f10(const std::sub_match<T1> &a0,
         const std::basic_string<typename std::iterator_traits<T1>::value_type,
                                 T2, T3> &a1) {
  return std::operator==(a0, a1);
}

template <typename T1, typename T2>
void f11(std::basic_regex<T1, T2> &a0, std::basic_regex<T1, T2> &a1) {
  return std::swap(a0, a1);
}

template <typename T1, typename T2>
void f12(std::match_results<T1, T2> &a0, std::match_results<T1, T2> &a1) {
  return std::swap(a0, a1);
}

using t1 = std::cmatch;

using t2 = std::cregex_iterator;

using t3 = std::cregex_token_iterator;

using t4 = std::csub_match;

using t5 = std::pmr::cmatch;

template <typename T1> using t6 = std::pmr::match_results<T1>;

using t7 = std::pmr::smatch;

using t8 = std::pmr::wcmatch;

using t9 = std::pmr::wsmatch;

using t10 = std::regex;

template <typename T1, typename T2>
bool f13(const T1 *a0, const std::basic_regex<T1, T2> &a1,
         std::regex_constants::match_flag_type a2) {
  return std::regex_match(a0, a1, a2);
}

template <typename T1, typename T2, typename T3>
bool f14(const T1 *a0, std::match_results<const T1 *, T2> &a1,
         const std::basic_regex<T1, T3> &a2,
         std::regex_constants::match_flag_type a3) {
  return std::regex_match(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
bool f15(T1 a0, T1 a1, const std::basic_regex<T2, T3> &a2,
         std::regex_constants::match_flag_type a3) {
  return std::regex_match(std::move(a0), std::move(a1), a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f16(const std::basic_string<T3, T1, T2> &a0,
         const std::basic_regex<T3, T4> &a1,
         std::regex_constants::match_flag_type a2) {
  return std::regex_match(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f17(T1 a0, T1 a1, std::match_results<T1, T2> &a2,
         const std::basic_regex<T3, T4> &a3,
         std::regex_constants::match_flag_type a4) {
  return std::regex_match(std::move(a0), std::move(a1), a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
bool f18(const std::basic_string<T4, T1, T2> &a0,
         std::match_results<
             typename std::basic_string<T4, T1, T2>::const_iterator, T3> &a1,
         const std::basic_regex<T4, T5> &a2,
         std::regex_constants::match_flag_type a3) {
  return std::regex_match(a0, a1, a2, a3);
}

template <typename T1, typename T2>
std::basic_string<T2> f19(const T2 *a0, const std::basic_regex<T2, T1> &a1,
                          const T2 *a2,
                          std::regex_constants::match_flag_type a3) {
  return std::regex_replace(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<T2, T3, T4>
f20(const std::basic_string<T2, T3, T4> &a0, const std::basic_regex<T2, T1> &a1,
    const T2 *a2, std::regex_constants::match_flag_type a3) {
  return std::regex_replace(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<T2> f21(const T2 *a0, const std::basic_regex<T2, T1> &a1,
                          const std::basic_string<T2, T3, T4> &a2,
                          std::regex_constants::match_flag_type a3) {
  return std::regex_replace(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
T1 f22(T1 a0, T2 a1, T2 a2, const std::basic_regex<T4, T3> &a3, const T4 *a4,
       std::regex_constants::match_flag_type a5) {
  return std::regex_replace(std::move(a0), std::move(a1), std::move(a2), a3, a4,
                            a5);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::basic_string<T2, T3, T4> f23(const std::basic_string<T2, T3, T4> &a0,
                                  const std::basic_regex<T2, T1> &a1,
                                  const std::basic_string<T2, T5, T6> &a2,
                                  std::regex_constants::match_flag_type a3) {
  return std::regex_replace(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
T1 f24(T1 a0, T2 a1, T2 a2, const std::basic_regex<T4, T3> &a3,
       const std::basic_string<T4, T5, T6> &a4,
       std::regex_constants::match_flag_type a5) {
  return std::regex_replace(std::move(a0), std::move(a1), std::move(a2), a3, a4,
                            a5);
}

template <typename T1, typename T2>
bool f25(const T1 *a0, const std::basic_regex<T1, T2> &a1,
         std::regex_constants::match_flag_type a2) {
  return std::regex_search(a0, a1, a2);
}

template <typename T1, typename T2, typename T3>
bool f26(const T1 *a0, std::match_results<const T1 *, T2> &a1,
         const std::basic_regex<T1, T3> &a2,
         std::regex_constants::match_flag_type a3) {
  return std::regex_search(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
bool f27(T1 a0, T1 a1, const std::basic_regex<T2, T3> &a2,
         std::regex_constants::match_flag_type a3) {
  return std::regex_search(std::move(a0), std::move(a1), a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f28(const std::basic_string<T3, T1, T2> &a0,
         const std::basic_regex<T3, T4> &a1,
         std::regex_constants::match_flag_type a2) {
  return std::regex_search(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f29(T1 a0, T1 a1, std::match_results<T1, T2> &a2,
         const std::basic_regex<T3, T4> &a3,
         std::regex_constants::match_flag_type a4) {
  return std::regex_search(std::move(a0), std::move(a1), a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
bool f30(const std::basic_string<T4, T1, T2> &a0,
         std::match_results<
             typename std::basic_string<T4, T1, T2>::const_iterator, T3> &a1,
         const std::basic_regex<T4, T5> &a2,
         std::regex_constants::match_flag_type a3) {
  return std::regex_search(a0, a1, a2, a3);
}

using t11 = std::smatch;

using t12 = std::sregex_iterator;

using t13 = std::sregex_token_iterator;

using t14 = std::ssub_match;

using t15 = std::wcmatch;

using t16 = std::wcregex_iterator;

using t17 = std::wcregex_token_iterator;

using t18 = std::wcsub_match;

using t19 = std::wregex;

using t20 = std::wsmatch;

using t21 = std::wsregex_iterator;

using t22 = std::wsregex_token_iterator;

using t23 = std::wssub_match;
