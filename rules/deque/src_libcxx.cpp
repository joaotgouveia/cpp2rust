#include <deque>
#include <initializer_list>
#include <utility>

template <typename T1, typename T2>
bool f81(const std::deque<T1, T2> &a0, const std::deque<T1, T2> &a1) {
  return std::operator== <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
void f82(std::deque<T1, T2> &a0, std::deque<T1, T2> &a1) {
  return std::swap<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          T5 T6>
using t15 = std::__deque_iterator<T1, T2, T3, T4, T5, T6>;

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          T5 T6>
using t16 = typename std::__deque_iterator<T1, T2, T3, T4, T5, T6>::pointer;

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          T5 T6>
using t17 =
    typename std::__deque_iterator<T1, T2, T3, T4, T5, T6>::difference_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          T5 T6>
using t18 = typename std::__deque_iterator<T1, T2, T3, T4, T5, T6>::value_type;

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          T5 T6>
using t19 =
    typename std::__deque_iterator<T1, T2, T3, T4, T5, T6>::iterator_category;

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          T5 T6>
using t20 = typename std::__deque_iterator<T1, T2, T3, T4, T5, T6>::reference;

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          T5 T6>
std::__deque_iterator<T1, T2, T3, T4, T5, T6> f83() {
  return std::__deque_iterator<T1, T2, T3, T4, T5, T6>();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          T5 T6>
T3 f84(const std::__deque_iterator<T1, T2, T3, T4, T5, T6> &a0) {
  return a0.operator*();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          T5 T6>
T2 f85(const std::__deque_iterator<T1, T2, T3, T4, T5, T6> &a0) {
  return a0.operator->();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          T5 T6>
std::__deque_iterator<T1, T2, T3, T4, T5, T6> &
f86(std::__deque_iterator<T1, T2, T3, T4, T5, T6> &a0) {
  return a0.operator++();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          T5 T6>
std::__deque_iterator<T1, T2, T3, T4, T5, T6>
f87(std::__deque_iterator<T1, T2, T3, T4, T5, T6> &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          T5 T6>
std::__deque_iterator<T1, T2, T3, T4, T5, T6> &
f88(std::__deque_iterator<T1, T2, T3, T4, T5, T6> &a0) {
  return a0.operator--();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          T5 T6>
std::__deque_iterator<T1, T2, T3, T4, T5, T6>
f89(std::__deque_iterator<T1, T2, T3, T4, T5, T6> &a0, int a1) {
  return a0.operator--(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          T5 T6>
std::__deque_iterator<T1, T2, T3, T4, T5, T6> &
f90(std::__deque_iterator<T1, T2, T3, T4, T5, T6> &a0, T5 a1) {
  return a0.operator+=(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          T5 T6>
std::__deque_iterator<T1, T2, T3, T4, T5, T6> &
f91(std::__deque_iterator<T1, T2, T3, T4, T5, T6> &a0, T5 a1) {
  return a0.operator-=(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          T5 T6>
std::__deque_iterator<T1, T2, T3, T4, T5, T6>
f92(const std::__deque_iterator<T1, T2, T3, T4, T5, T6> &a0, T5 a1) {
  return a0.operator+(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          T5 T6>
std::__deque_iterator<T1, T2, T3, T4, T5, T6>
f93(const std::__deque_iterator<T1, T2, T3, T4, T5, T6> &a0, T5 a1) {
  return a0.operator-(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          T5 T6>
T3 f94(const std::__deque_iterator<T1, T2, T3, T4, T5, T6> &a0, T5 a1) {
  return a0.operator[](std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          T5 T6, typename T7, typename T8, typename T9,
          std::__enable_if_t<std::is_convertible<T7, T2>::value, int> T10 = 0>
std::__deque_iterator<T1, T2, T3, T4, T5, T6>
f95(const std::__deque_iterator<T1, T7, T8, T9, T5, T6> &a0) {
  return std::__deque_iterator<T1, T2, T3, T4, T5, T6>(a0);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          T5 T6>
bool f96(const std::__deque_iterator<T1, T2, T3, T4, T5, T6> &a0,
         const std::__deque_iterator<T1, T2, T3, T4, T5, T6> &a1) {
  return operator==(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          T5 T6>
T5 f97(const std::__deque_iterator<T1, T2, T3, T4, T5, T6> &a0,
       const std::__deque_iterator<T1, T2, T3, T4, T5, T6> &a1) {
  return operator-(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          T5 T6>
std::__deque_iterator<T1, T2, T3, T4, T5, T6>
f98(T5 a0, const std::__deque_iterator<T1, T2, T3, T4, T5, T6> &a1) {
  return operator+(std::move(a0), a1);
}

template <typename T1, typename T2>
std::deque<T1, T2> f99(typename std::deque<T1, T2>::size_type a0) {
  return std::deque<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
std::deque<T1, T2> f100(typename std::deque<T1, T2>::size_type a0,
                        const T2 &a1) {
  return std::deque<T1, T2>(std::move(a0), a1);
}

template <typename T1, typename T2>
std::deque<T1, T2> f101(typename std::deque<T1, T2>::size_type a0,
                        const T1 &a1) {
  return std::deque<T1, T2>(std::move(a0), a1);
}

template <typename T1, typename T2>
std::deque<T1, T2> f102(std::initializer_list<T1> a0) {
  return std::deque<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
void f103(std::deque<T1, T2> &a0, typename std::deque<T1, T2>::size_type a1,
          const T1 &a2) {
  return a0.assign(std::move(a1), a2);
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::size_type f104(const std::deque<T1, T2> &a0) {
  return a0.size();
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::size_type f105(const std::deque<T1, T2> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2>
void f106(std::deque<T1, T2> &a0, typename std::deque<T1, T2>::size_type a1) {
  return a0.resize(std::move(a1));
}

template <typename T1, typename T2>
void f107(std::deque<T1, T2> &a0, typename std::deque<T1, T2>::size_type a1,
          const T1 &a2) {
  return a0.resize(std::move(a1), a2);
}

template <typename T1, typename T2>
T1 &f108(std::deque<T1, T2> &a0, typename std::deque<T1, T2>::size_type a1) {
  return a0.operator[](std::move(a1));
}

template <typename T1, typename T2>
const T1 &f109(const std::deque<T1, T2> &a0,
               typename std::deque<T1, T2>::size_type a1) {
  return a0.operator[](std::move(a1));
}

template <typename T1, typename T2>
T1 &f110(std::deque<T1, T2> &a0, typename std::deque<T1, T2>::size_type a1) {
  return a0.at(std::move(a1));
}

template <typename T1, typename T2>
const T1 &f111(const std::deque<T1, T2> &a0,
               typename std::deque<T1, T2>::size_type a1) {
  return a0.at(std::move(a1));
}

template <typename T1, typename T2> T1 &f112(std::deque<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
const T1 &f113(const std::deque<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2> T1 &f114(std::deque<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
const T1 &f115(const std::deque<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
typename std::deque<T1, T2>::iterator
f116(std::deque<T1, T2> &a0, typename std::deque<T1, T2>::const_iterator a1,
     typename std::deque<T1, T2>::size_type a2, const T1 &a3) {
  return a0.insert(std::move(a1), std::move(a2), a3);
}

template <typename T1, typename T2,
          std::__enable_if_t<std::__is_allocator_v<T2>, int> T3 = 0>
std::deque<T1, T2> f117(typename std::deque<T1, T2>::size_type a0, const T1 &a1,
                        const T2 &a2) {
  return std::deque<T1, T2>(std::move(a0), a1, a2);
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<std::__has_input_iterator_category<T3>::value, int>
              T4 = 0>
std::deque<T1, T2> f118(T3 a0, T3 a1) {
  return std::deque<T1, T2>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<std::__has_input_iterator_category<T3>::value, int>
              T4 = 0>
std::deque<T1, T2> f119(T3 a0, T3 a1, const T2 &a2) {
  return std::deque<T1, T2>(std::move(a0), std::move(a1), a2);
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<
              std::__has_input_iterator_category<T3>::value &&
                  !std::__has_random_access_iterator_category<T3>::value,
              int>
              T4 = 0>
void f120(std::deque<T1, T2> &a0, T3 a1, T3 a2) {
  return a0.template assign<T3>(std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<
              std::__has_random_access_iterator_category<T3>::value, int>
              T4 = 0>
void f121(std::deque<T1, T2> &a0, T3 a1, T3 a2) {
  return a0.template assign<T3>(std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<
              std::__has_exactly_input_iterator_category<T3>::value, int>
              T4 = 0>
typename std::deque<T1, T2>::iterator
f122(std::deque<T1, T2> &a0, typename std::deque<T1, T2>::const_iterator a1,
     T3 a2, T3 a3) {
  return a0.template insert<T3>(std::move(a1), std::move(a2), std::move(a3));
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<
              std::__has_exactly_forward_iterator_category<T3>::value, int>
              T4 = 0>
typename std::deque<T1, T2>::iterator
f123(std::deque<T1, T2> &a0, typename std::deque<T1, T2>::const_iterator a1,
     T3 a2, T3 a3) {
  return a0.template insert<T3>(std::move(a1), std::move(a2), std::move(a3));
}

template <typename T1, typename T2, typename T3,
          std::__enable_if_t<
              std::__has_bidirectional_iterator_category<T3>::value, int>
              T4 = 0>
typename std::deque<T1, T2>::iterator
f124(std::deque<T1, T2> &a0, typename std::deque<T1, T2>::const_iterator a1,
     T3 a2, T3 a3) {
  return a0.template insert<T3>(std::move(a1), std::move(a2), std::move(a3));
}
