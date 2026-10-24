#include <iterator>
#include <ostream>
#include <regex>
#include <string>
#include <utility>

template <typename T1, typename T2, typename T3>
std::basic_ostream<T1, T2> &f40(std::basic_ostream<T1, T2> &a0,
                                const std::sub_match<T3> &a1) {
  return std::operator<< <T1, T2, T3>(a0, a1);
}

template <typename T1>
bool f41(const std::sub_match<T1> &a0,
         const typename std::iterator_traits<T1>::value_type &a1) {
  return std::operator== <T1>(a0, a1);
}

template <typename T1>
bool f42(const std::sub_match<T1> &a0,
         const typename std::iterator_traits<T1>::value_type *a1) {
  return std::operator== <T1>(a0, a1);
}

template <typename T1>
bool f43(const std::sub_match<T1> &a0, const std::sub_match<T1> &a1) {
  return std::operator== <T1>(a0, a1);
}

template <typename T1, typename T2>
bool f44(const std::match_results<T1, T2> &a0,
         const std::match_results<T1, T2> &a1) {
  return std::operator== <T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f45(const std::sub_match<T1> &a0,
         const std::basic_string<typename std::iterator_traits<T1>::value_type,
                                 T2, T3> &a1) {
  return std::operator== <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2>
void f46(std::basic_regex<T1, T2> &a0, std::basic_regex<T1, T2> &a1) {
  return std::swap<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
void f47(std::match_results<T1, T2> &a0, std::match_results<T1, T2> &a1) {
  return std::swap<T1, T2>(a0, a1);
}

using t1 = std::cmatch;

using t2 = std::cregex_iterator;

using t3 = std::cregex_token_iterator;

using t4 = std::csub_match;

using t5 = std::regex;

template <typename T1, typename T2>
bool f48(const T1 *a0, const std::basic_regex<T1, T2> &a1,
         std::regex_constants::match_flag_type a2) {
  return std::regex_match<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3>
bool f49(const T1 *a0, std::match_results<const T1 *, T2> &a1,
         const std::basic_regex<T1, T3> &a2,
         std::regex_constants::match_flag_type a3) {
  return std::regex_match<T1, T2, T3>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
bool f50(T1 a0, T1 a1, const std::basic_regex<T2, T3> &a2,
         std::regex_constants::match_flag_type a3) {
  return std::regex_match<T1, T2, T3>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f51(const std::basic_string<T3, T1, T2> &a0,
         const std::basic_regex<T3, T4> &a1,
         std::regex_constants::match_flag_type a2) {
  return std::regex_match<T1, T2, T3, T4>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f52(T1 a0, T1 a1, std::match_results<T1, T2> &a2,
         const std::basic_regex<T3, T4> &a3,
         std::regex_constants::match_flag_type a4) {
  return std::regex_match<T1, T2, T3, T4>(a0, a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
bool f53(const std::basic_string<T4, T1, T2> &a0,
         std::match_results<
             typename std::basic_string<T4, T1, T2>::const_iterator, T3> &a1,
         const std::basic_regex<T4, T5> &a2,
         std::regex_constants::match_flag_type a3) {
  return std::regex_match<T1, T2, T3, T4, T5>(a0, a1, a2, a3);
}

template <typename T1, typename T2>
std::basic_string<T2> f54(const T2 *a0, const std::basic_regex<T2, T1> &a1,
                          const T2 *a2,
                          std::regex_constants::match_flag_type a3) {
  return std::regex_replace<T1, T2>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<T2, T3, T4>
f55(const std::basic_string<T2, T3, T4> &a0, const std::basic_regex<T2, T1> &a1,
    const T2 *a2, std::regex_constants::match_flag_type a3) {
  return std::regex_replace<T1, T2, T3, T4>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<T2> f56(const T2 *a0, const std::basic_regex<T2, T1> &a1,
                          const std::basic_string<T2, T3, T4> &a2,
                          std::regex_constants::match_flag_type a3) {
  return std::regex_replace<T1, T2, T3, T4>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
T1 f57(T1 a0, T2 a1, T2 a2, const std::basic_regex<T4, T3> &a3, const T4 *a4,
       std::regex_constants::match_flag_type a5) {
  return std::regex_replace<T1, T2, T3, T4>(a0, a1, a2, a3, a4, a5);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::basic_string<T2, T3, T4> f58(const std::basic_string<T2, T3, T4> &a0,
                                  const std::basic_regex<T2, T1> &a1,
                                  const std::basic_string<T2, T5, T6> &a2,
                                  std::regex_constants::match_flag_type a3) {
  return std::regex_replace<T1, T2, T3, T4, T5, T6>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
T1 f59(T1 a0, T2 a1, T2 a2, const std::basic_regex<T4, T3> &a3,
       const std::basic_string<T4, T5, T6> &a4,
       std::regex_constants::match_flag_type a5) {
  return std::regex_replace<T1, T2, T3, T4, T5, T6>(a0, a1, a2, a3, a4, a5);
}

template <typename T1, typename T2>
bool f60(const T1 *a0, const std::basic_regex<T1, T2> &a1,
         std::regex_constants::match_flag_type a2) {
  return std::regex_search<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3>
bool f61(const T1 *a0, std::match_results<const T1 *, T2> &a1,
         const std::basic_regex<T1, T3> &a2,
         std::regex_constants::match_flag_type a3) {
  return std::regex_search<T1, T2, T3>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3>
bool f62(T1 a0, T1 a1, const std::basic_regex<T2, T3> &a2,
         std::regex_constants::match_flag_type a3) {
  return std::regex_search<T1, T2, T3>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f63(const std::basic_string<T3, T1, T2> &a0,
         const std::basic_regex<T3, T4> &a1,
         std::regex_constants::match_flag_type a2) {
  return std::regex_search<T1, T2, T3, T4>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f64(T1 a0, T1 a1, std::match_results<T1, T2> &a2,
         const std::basic_regex<T3, T4> &a3,
         std::regex_constants::match_flag_type a4) {
  return std::regex_search<T1, T2, T3, T4>(a0, a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
bool f65(const std::basic_string<T4, T1, T2> &a0,
         std::match_results<
             typename std::basic_string<T4, T1, T2>::const_iterator, T3> &a1,
         const std::basic_regex<T4, T5> &a2,
         std::regex_constants::match_flag_type a3) {
  return std::regex_search<T1, T2, T3, T4, T5>(a0, a1, a2, a3);
}

using t6 = std::smatch;

using t7 = std::sregex_iterator;

using t8 = std::sregex_token_iterator;

using t9 = std::ssub_match;

using t10 = std::wcmatch;

using t11 = std::wcregex_iterator;

using t12 = std::wcregex_token_iterator;

using t13 = std::wcsub_match;

using t14 = std::wregex;

using t15 = std::wsmatch;

using t16 = std::wsregex_iterator;

using t17 = std::wsregex_token_iterator;

using t18 = std::wssub_match;
