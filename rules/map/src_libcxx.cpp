#include <initializer_list>
#include <map>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
bool f120(const std::map<T1, T2, T3, T4> &a0,
          const std::map<T1, T2, T3, T4> &a1) {
  return std::operator== <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f121(const std::multimap<T1, T2, T3, T4> &a0,
          const std::multimap<T1, T2, T3, T4> &a1) {
  return std::operator== <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
void f122(std::map<T1, T2, T3, T4> &a0, std::map<T1, T2, T3, T4> &a1) {
  return std::swap<T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
void f123(std::multimap<T1, T2, T3, T4> &a0,
          std::multimap<T1, T2, T3, T4> &a1) {
  return std::swap<T1, T2, T3, T4>(a0, a1);
}

template <typename T1> using t26 = std::__map_const_iterator<T1>;

template <typename T1>
using t27 = typename std::__map_const_iterator<T1>::iterator_category;

template <typename T1>
using t28 = typename std::__map_const_iterator<T1>::value_type;

template <typename T1>
using t29 = typename std::__map_const_iterator<T1>::difference_type;

template <typename T1>
using t30 = typename std::__map_const_iterator<T1>::reference;

template <typename T1>
using t31 = typename std::__map_const_iterator<T1>::pointer;

template <typename T1> std::__map_const_iterator<T1> f124() {
  return std::__map_const_iterator<T1>();
}

template <typename T1> std::__map_const_iterator<T1> f125(T1 a0) {
  return std::__map_const_iterator<T1>(a0);
}

template <typename T1>
const typename std::__map_const_iterator<T1>::value_type &
f126(const std::__map_const_iterator<T1> &a0) {
  return a0.operator*();
}

template <typename T1>
typename std::__map_const_iterator<T1>::pointer
f127(const std::__map_const_iterator<T1> &a0) {
  return a0.operator->();
}

template <typename T1>
std::__map_const_iterator<T1> &f128(std::__map_const_iterator<T1> &a0) {
  return a0.operator++();
}

template <typename T1>
std::__map_const_iterator<T1> f129(std::__map_const_iterator<T1> &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1>
std::__map_const_iterator<T1> &f130(std::__map_const_iterator<T1> &a0) {
  return a0.operator--();
}

template <typename T1>
std::__map_const_iterator<T1> f131(std::__map_const_iterator<T1> &a0, int a1) {
  return a0.operator--(a1);
}

template <typename T1>
bool f132(const std::__map_const_iterator<T1> &a0,
          const std::__map_const_iterator<T1> &a1) {
  return operator!=(a0, a1);
}

template <typename T1>
bool f133(const std::__map_const_iterator<T1> &a0,
          const std::__map_const_iterator<T1> &a1) {
  return operator==(a0, a1);
}

template <typename T1> using t32 = std::__map_iterator<T1>;

template <typename T1>
using t33 = typename std::__map_iterator<T1>::iterator_category;

template <typename T1> using t34 = typename std::__map_iterator<T1>::value_type;

template <typename T1>
using t35 = typename std::__map_iterator<T1>::difference_type;

template <typename T1> using t36 = typename std::__map_iterator<T1>::reference;

template <typename T1> using t37 = typename std::__map_iterator<T1>::pointer;

template <typename T1> std::__map_iterator<T1> f134() {
  return std::__map_iterator<T1>();
}

template <typename T1> std::__map_iterator<T1> f135(T1 a0) {
  return std::__map_iterator<T1>(a0);
}

template <typename T1>
typename std::__map_iterator<T1>::value_type &
f136(const std::__map_iterator<T1> &a0) {
  return a0.operator*();
}

template <typename T1>
typename std::__map_iterator<T1>::pointer
f137(const std::__map_iterator<T1> &a0) {
  return a0.operator->();
}

template <typename T1>
std::__map_iterator<T1> &f138(std::__map_iterator<T1> &a0) {
  return a0.operator++();
}

template <typename T1>
std::__map_iterator<T1> f139(std::__map_iterator<T1> &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1>
std::__map_iterator<T1> &f140(std::__map_iterator<T1> &a0) {
  return a0.operator--();
}

template <typename T1>
std::__map_iterator<T1> f141(std::__map_iterator<T1> &a0, int a1) {
  return a0.operator--(a1);
}

template <typename T1>
bool f142(const std::__map_iterator<T1> &a0,
          const std::__map_iterator<T1> &a1) {
  return operator!=(a0, a1);
}

template <typename T1>
bool f143(const std::__map_iterator<T1> &a0,
          const std::__map_iterator<T1> &a1) {
  return operator==(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::map<T1, T2, T3, T4> f144(const T3 &a0) {
  return std::map<T1, T2, T3, T4>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::map<T1, T2, T3, T4> f145(std::initializer_list<std::pair<const T1, T2>> a0,
                              const T3 &a1) {
  return std::map<T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::map<T1, T2, T3, T4> f146(T5 a0, T5 a1, const T3 &a2) {
  return std::map<T1, T2, T3, T4>(a0, a1, a2);
}
