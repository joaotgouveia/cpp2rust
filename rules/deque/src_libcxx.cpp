#include <deque>
#include <initializer_list>
#include <utility>

template <typename T1, typename T2>
bool f76(const std::deque<T1, T2> &a0, const std::deque<T1, T2> &a1) {
  return std::operator== <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
void f77(std::deque<T1, T2> &a0, std::deque<T1, T2> &a1) {
  return std::swap<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::deque<T1, T2> f78(typename std::deque<T1, T2>::size_type a0) {
  return std::deque<T1, T2>(a0);
}

template <typename T1, typename T2>
std::deque<T1, T2> f79(typename std::deque<T1, T2>::size_type a0,
                       const T2 &a1) {
  return std::deque<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::deque<T1, T2> f80(typename std::deque<T1, T2>::size_type a0,
                       const T1 &a1) {
  return std::deque<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::deque<T1, T2> f81(std::initializer_list<T1> a0) {
  return std::deque<T1, T2>(a0);
}

template <typename T1, typename T2>
void f82(std::deque<T1, T2> &a0, typename std::deque<T1, T2>::size_type a1,
         const T1 &a2) {
  return a0.assign(a1, a2);
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::size_type f83(const std::deque<T1, T2> &a0) {
  return a0.size();
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::size_type f84(const std::deque<T1, T2> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2>
void f85(std::deque<T1, T2> &a0, typename std::deque<T1, T2>::size_type a1) {
  return a0.resize(a1);
}

template <typename T1, typename T2>
void f86(std::deque<T1, T2> &a0, typename std::deque<T1, T2>::size_type a1,
         const T1 &a2) {
  return a0.resize(a1, a2);
}

template <typename T1, typename T2>
T1 &f87(std::deque<T1, T2> &a0, typename std::deque<T1, T2>::size_type a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2>
const T1 &f88(const std::deque<T1, T2> &a0,
              typename std::deque<T1, T2>::size_type a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2>
T1 &f89(std::deque<T1, T2> &a0, typename std::deque<T1, T2>::size_type a1) {
  return a0.at(a1);
}

template <typename T1, typename T2>
const T1 &f90(const std::deque<T1, T2> &a0,
              typename std::deque<T1, T2>::size_type a1) {
  return a0.at(a1);
}

template <typename T1, typename T2> T1 &f91(std::deque<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
const T1 &f92(const std::deque<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2> T1 &f93(std::deque<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
const T1 &f94(const std::deque<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::iterator
f95(std::deque<T1, T2> &a0, typename std::deque<T1, T2>::const_iterator a1,
    typename std::deque<T1, T2>::size_type a2, const T1 &a3) {
  return a0.insert(a1, a2, a3);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_allocator_v<T2>, int> T3 = 0>
std::deque<T1, T2> f96(typename std::deque<T1, T2>::size_type a0, const T1 &a1,
                       const T2 &a2) {
  return std::deque<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<std::__has_input_iterator_category<T3>::value, int>
              T4 = 0>
std::deque<T1, T2> f97(T3 a0, T3 a1) {
  return std::deque<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<std::__has_input_iterator_category<T3>::value, int>
              T4 = 0>
std::deque<T1, T2> f98(T3 a0, T3 a1, const T2 &a2) {
  return std::deque<T1, T2>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<
              std::__has_input_iterator_category<T3>::value &&
                  !std::__has_random_access_iterator_category<T3>::value,
              int>
              T4 = 0>
void f99(std::deque<T1, T2> &a0, T3 a1, T3 a2) {
  return a0.template assign<T3>(a1, a2);
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<
              std::__has_random_access_iterator_category<T3>::value, int>
              T4 = 0>
void f100(std::deque<T1, T2> &a0, T3 a1, T3 a2) {
  return a0.template assign<T3>(a1, a2);
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<
              std::__has_exactly_input_iterator_category<T3>::value, int>
              T4 = 0>
typename std::deque<T1, T2>::iterator
f101(std::deque<T1, T2> &a0, typename std::deque<T1, T2>::const_iterator a1,
     T3 a2, T3 a3) {
  return a0.template insert<T3>(a1, a2, a3);
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<
              std::__has_exactly_forward_iterator_category<T3>::value, int>
              T4 = 0>
typename std::deque<T1, T2>::iterator
f102(std::deque<T1, T2> &a0, typename std::deque<T1, T2>::const_iterator a1,
     T3 a2, T3 a3) {
  return a0.template insert<T3>(a1, a2, a3);
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<
              std::__has_bidirectional_iterator_category<T3>::value, int>
              T4 = 0>
typename std::deque<T1, T2>::iterator
f103(std::deque<T1, T2> &a0, typename std::deque<T1, T2>::const_iterator a1,
     T3 a2, T3 a3) {
  return a0.template insert<T3>(a1, a2, a3);
}
