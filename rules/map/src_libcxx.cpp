#include <initializer_list>
#include <map>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
bool f121(const std::map<T1, T2, T3, T4> &a0,
          const std::map<T1, T2, T3, T4> &a1) {
  return std::operator== <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
bool f122(const std::multimap<T1, T2, T3, T4> &a0,
          const std::multimap<T1, T2, T3, T4> &a1) {
  return std::operator== <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
void f123(std::map<T1, T2, T3, T4> &a0, std::map<T1, T2, T3, T4> &a1) {
  return std::swap<T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
void f124(std::multimap<T1, T2, T3, T4> &a0,
          std::multimap<T1, T2, T3, T4> &a1) {
  return std::swap<T1, T2, T3, T4>(a0, a1);
}

template <typename T1> using t32 = std::__map_const_iterator<T1>;

template <typename T1>
using t33 = typename std::__map_const_iterator<T1>::iterator_category;

template <typename T1>
using t34 = typename std::__map_const_iterator<T1>::value_type;

template <typename T1>
using t35 = typename std::__map_const_iterator<T1>::difference_type;

template <typename T1>
using t36 = typename std::__map_const_iterator<T1>::reference;

template <typename T1>
using t37 = typename std::__map_const_iterator<T1>::pointer;

template <typename T1> std::__map_const_iterator<T1> f125() {
  return std::__map_const_iterator<T1>();
}

template <typename T1> std::__map_const_iterator<T1> f126(T1 a0) {
  return std::__map_const_iterator<T1>(a0);
}

template <typename T1>
std::__map_const_iterator<T1>
f127(std::__map_iterator<typename T1::__non_const_iterator> a0) {
  return std::__map_const_iterator<T1>(a0);
}

template <typename T1>
const typename std::__map_const_iterator<T1>::value_type &
f128(const std::__map_const_iterator<T1> &a0) {
  return a0.operator*();
}

template <typename T1>
typename std::__map_const_iterator<T1>::pointer
f129(const std::__map_const_iterator<T1> &a0) {
  return a0.operator->();
}

template <typename T1>
std::__map_const_iterator<T1> &f130(std::__map_const_iterator<T1> &a0) {
  return a0.operator++();
}

template <typename T1>
std::__map_const_iterator<T1> f131(std::__map_const_iterator<T1> &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1>
std::__map_const_iterator<T1> &f132(std::__map_const_iterator<T1> &a0) {
  return a0.operator--();
}

template <typename T1>
std::__map_const_iterator<T1> f133(std::__map_const_iterator<T1> &a0, int a1) {
  return a0.operator--(a1);
}

template <typename T1>
bool f134(const std::__map_const_iterator<T1> &a0,
          const std::__map_const_iterator<T1> &a1) {
  return operator!=(a0, a1);
}

template <typename T1>
bool f135(const std::__map_const_iterator<T1> &a0,
          const std::__map_const_iterator<T1> &a1) {
  return operator==(a0, a1);
}

template <typename T1> using t38 = std::__map_iterator<T1>;

template <typename T1>
using t39 = typename std::__map_iterator<T1>::iterator_category;

template <typename T1> using t40 = typename std::__map_iterator<T1>::value_type;

template <typename T1>
using t41 = typename std::__map_iterator<T1>::difference_type;

template <typename T1> using t42 = typename std::__map_iterator<T1>::reference;

template <typename T1> using t43 = typename std::__map_iterator<T1>::pointer;

template <typename T1> std::__map_iterator<T1> f136() {
  return std::__map_iterator<T1>();
}

template <typename T1> std::__map_iterator<T1> f137(T1 a0) {
  return std::__map_iterator<T1>(a0);
}

template <typename T1>
typename std::__map_iterator<T1>::value_type &
f138(const std::__map_iterator<T1> &a0) {
  return a0.operator*();
}

template <typename T1>
typename std::__map_iterator<T1>::pointer
f139(const std::__map_iterator<T1> &a0) {
  return a0.operator->();
}

template <typename T1>
std::__map_iterator<T1> &f140(std::__map_iterator<T1> &a0) {
  return a0.operator++();
}

template <typename T1>
std::__map_iterator<T1> f141(std::__map_iterator<T1> &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1>
std::__map_iterator<T1> &f142(std::__map_iterator<T1> &a0) {
  return a0.operator--();
}

template <typename T1>
std::__map_iterator<T1> f143(std::__map_iterator<T1> &a0, int a1) {
  return a0.operator--(a1);
}

template <typename T1>
bool f144(const std::__map_iterator<T1> &a0,
          const std::__map_iterator<T1> &a1) {
  return operator!=(a0, a1);
}

template <typename T1>
bool f145(const std::__map_iterator<T1> &a0,
          const std::__map_iterator<T1> &a1) {
  return operator==(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::map<T1, T2, T3, T4> f146(const T3 &a0) {
  return std::map<T1, T2, T3, T4>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::map<T1, T2, T3, T4> f147(std::initializer_list<std::pair<const T1, T2>> a0,
                              const T3 &a1) {
  return std::map<T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::map<T1, T2, T3, T4> f148(T5 a0, T5 a1, const T3 &a2) {
  return std::map<T1, T2, T3, T4>(a0, a1, a2);
}
