#include <regex>
#include <string>
#include <utility>

template <typename T1, typename T2>
std::match_results<T1, T2> f22(const T2 &a0) {
  return std::match_results<T1, T2>(a0);
}

template <typename T1, typename T2>
std::match_results<T1, T2> f23(const std::match_results<T1, T2> &a0) {
  return std::match_results<T1, T2>(a0);
}

template <typename T1, typename T2>
std::match_results<T1, T2> f24(std::match_results<T1, T2> &&a0) {
  return std::match_results<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::match_results<T1, T2> &f25(std::match_results<T1, T2> &a0,
                                const std::match_results<T1, T2> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
std::match_results<T1, T2> &f26(std::match_results<T1, T2> &a0,
                                std::match_results<T1, T2> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2>
std::match_results<T1, T2> f27(const std::match_results<T1, T2> &a0,
                               const T2 &a1) {
  return std::match_results<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::match_results<T1, T2> f28(std::match_results<T1, T2> &&a0, const T2 &a1) {
  return std::match_results<T1, T2>(std::move(a0), a1);
}

template <typename T1, typename T2>
using t35 = typename std::match_results<T1, T2>::const_iterator::iterator_type;

template <typename T1, typename T2>
using t36 = typename std::match_results<T1, T2>::iterator::iterator_type;
