#include <algorithm>
#include <initializer_list>
#include <regex>
#include <utility>
#include <vector>

template <typename T1, typename T2, typename T3>
using t1 = std::regex_token_iterator<T1, T2, T3>;

template <typename T1, typename T2, typename T3>
using t2 = typename std::regex_token_iterator<T1, T2, T3>::regex_type;

template <typename T1, typename T2, typename T3>
using t3 = typename std::regex_token_iterator<T1, T2, T3>::value_type;

template <typename T1, typename T2, typename T3>
using t4 = typename std::regex_token_iterator<T1, T2, T3>::difference_type;

template <typename T1, typename T2, typename T3>
using t5 = typename std::regex_token_iterator<T1, T2, T3>::pointer;

template <typename T1, typename T2, typename T3>
using t6 = typename std::regex_token_iterator<T1, T2, T3>::reference;

template <typename T1, typename T2, typename T3>
using t7 = typename std::regex_token_iterator<T1, T2, T3>::iterator_category;

template <typename T1, typename T2, typename T3>
using t8 = typename std::regex_token_iterator<T1, T2, T3>::iterator_concept;

template <typename T1, typename T2, typename T3>
std::regex_token_iterator<T1, T2, T3> f1() {
  return std::regex_token_iterator<T1, T2, T3>();
}

template <typename T1, typename T2, typename T3>
std::regex_token_iterator<T1, T2, T3>
f2(T1 a0, T1 a1, const std::basic_regex<T2, T3> &a2, int a3,
   std::regex_constants::match_flag_type a4) {
  return std::regex_token_iterator<T1, T2, T3>(std::move(a0), std::move(a1), a2,
                                               a3, a4);
}

template <typename T1, typename T2, typename T3>
std::regex_token_iterator<T1, T2, T3>
f3(T1 a0, T1 a1, const std::basic_regex<T2, T3> &a2, const std::vector<int> &a3,
   std::regex_constants::match_flag_type a4) {
  return std::regex_token_iterator<T1, T2, T3>(std::move(a0), std::move(a1), a2,
                                               a3, a4);
}

template <typename T1, typename T2, typename T3>
std::regex_token_iterator<T1, T2, T3>
f4(T1 a0, T1 a1, const std::basic_regex<T2, T3> &a2,
   std::initializer_list<int> a3, std::regex_constants::match_flag_type a4) {
  return std::regex_token_iterator<T1, T2, T3>(std::move(a0), std::move(a1), a2,
                                               std::move(a3), a4);
}

template <typename T1, typename T2, typename T3>
std::regex_token_iterator<T1, T2, T3>
f5(const std::regex_token_iterator<T1, T2, T3> &a0) {
  return std::regex_token_iterator<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
std::regex_token_iterator<T1, T2, T3> &
f6(std::regex_token_iterator<T1, T2, T3> &a0,
   const std::regex_token_iterator<T1, T2, T3> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3>
bool f7(const std::regex_token_iterator<T1, T2, T3> &a0,
        const std::regex_token_iterator<T1, T2, T3> &a1) {
  return a0.operator==(a1);
}

template <typename T1, typename T2, typename T3>
bool f8(const std::regex_token_iterator<T1, T2, T3> &a0,
        std::default_sentinel_t a1) {
  return a0.operator==(std::move(a1));
}

template <typename T1, typename T2, typename T3>
const std::sub_match<T1> &f9(const std::regex_token_iterator<T1, T2, T3> &a0) {
  return a0.operator*();
}

template <typename T1, typename T2, typename T3>
const std::sub_match<T1> *f10(const std::regex_token_iterator<T1, T2, T3> &a0) {
  return a0.operator->();
}

template <typename T1, typename T2, typename T3>
std::regex_token_iterator<T1, T2, T3> &
f11(std::regex_token_iterator<T1, T2, T3> &a0) {
  return a0.operator++();
}

template <typename T1, typename T2, typename T3>
std::regex_token_iterator<T1, T2, T3>
f12(std::regex_token_iterator<T1, T2, T3> &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1, typename T2, typename T3, std::size_t T4>
std::regex_token_iterator<T1, T2, T3>
f13(T1 a0, T1 a1, const std::basic_regex<T2, T3> &a2, const int (&a3)[T4],
    std::regex_constants::match_flag_type a4) {
  return std::regex_token_iterator<T1, T2, T3>(std::move(a0), std::move(a1), a2,
                                               a3, a4);
}

template <typename T1, typename T2, typename T3>
auto &f14(std::regex_token_iterator<T1, T2, T3> &a0) {
  return a0->matched;
}
