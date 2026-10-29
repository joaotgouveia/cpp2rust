#include <algorithm>
#include <regex>
#include <utility>

template <typename T1, typename T2, typename T3>
using t1 = std::regex_iterator<T1, T2, T3>;

template <typename T1, typename T2, typename T3>
using t2 = typename std::regex_iterator<T1, T2, T3>::regex_type;

template <typename T1, typename T2, typename T3>
using t3 = typename std::regex_iterator<T1, T2, T3>::value_type;

template <typename T1, typename T2, typename T3>
using t4 = typename std::regex_iterator<T1, T2, T3>::difference_type;

template <typename T1, typename T2, typename T3>
using t5 = typename std::regex_iterator<T1, T2, T3>::pointer;

template <typename T1, typename T2, typename T3>
using t6 = typename std::regex_iterator<T1, T2, T3>::reference;

template <typename T1, typename T2, typename T3>
using t7 = typename std::regex_iterator<T1, T2, T3>::iterator_category;

template <typename T1, typename T2, typename T3>
using t8 = typename std::regex_iterator<T1, T2, T3>::iterator_concept;

template <typename T1, typename T2, typename T3>
std::regex_iterator<T1, T2, T3> f1() {
  return std::regex_iterator<T1, T2, T3>();
}

template <typename T1, typename T2, typename T3>
std::regex_iterator<T1, T2, T3> f2(T1 a0, T1 a1,
                                   const std::basic_regex<T2, T3> &a2,
                                   std::regex_constants::match_flag_type a3) {
  return std::regex_iterator<T1, T2, T3>(std::move(a0), std::move(a1), a2, a3);
}

template <typename T1, typename T2, typename T3>
std::regex_iterator<T1, T2, T3> f3(const std::regex_iterator<T1, T2, T3> &a0) {
  return std::regex_iterator<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
std::regex_iterator<T1, T2, T3> &f4(std::regex_iterator<T1, T2, T3> &a0,
                                    const std::regex_iterator<T1, T2, T3> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3>
bool f5(const std::regex_iterator<T1, T2, T3> &a0,
        const std::regex_iterator<T1, T2, T3> &a1) {
  return a0.operator==(a1);
}

template <typename T1, typename T2, typename T3>
bool f6(const std::regex_iterator<T1, T2, T3> &a0, std::default_sentinel_t a1) {
  return a0.operator==(std::move(a1));
}

template <typename T1, typename T2, typename T3>
const std::match_results<T1> &f7(const std::regex_iterator<T1, T2, T3> &a0) {
  return a0.operator*();
}

template <typename T1, typename T2, typename T3>
const std::match_results<T1> *f8(const std::regex_iterator<T1, T2, T3> &a0) {
  return a0.operator->();
}

template <typename T1, typename T2, typename T3>
std::regex_iterator<T1, T2, T3> &f9(std::regex_iterator<T1, T2, T3> &a0) {
  return a0.operator++();
}

template <typename T1, typename T2, typename T3>
std::regex_iterator<T1, T2, T3> f10(std::regex_iterator<T1, T2, T3> &a0,
                                    int a1) {
  return a0.operator++(a1);
}
