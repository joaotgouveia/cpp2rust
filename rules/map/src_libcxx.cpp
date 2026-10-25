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

template <typename T1> using t22 = std::__map_const_iterator<T1>;

template <typename T1>
using t23 = typename std::__map_const_iterator<T1>::iterator_category;

template <typename T1>
using t24 = typename std::__map_const_iterator<T1>::value_type;

template <typename T1>
using t25 = typename std::__map_const_iterator<T1>::difference_type;

template <typename T1>
using t26 = typename std::__map_const_iterator<T1>::reference;

template <typename T1>
using t27 = typename std::__map_const_iterator<T1>::pointer;

template <typename T1> std::__map_const_iterator<T1> f125() {
  return std::__map_const_iterator<T1>();
}

template <typename T1> std::__map_const_iterator<T1> f126(T1 a0) {
  return std::__map_const_iterator<T1>(std::move(a0));
}

template <typename T1>
std::__map_const_iterator<T1>
f127(std::__map_iterator<typename T1::__non_const_iterator> a0) {
  return std::__map_const_iterator<T1>(std::move(a0));
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

template <typename T1> using t28 = std::__map_iterator<T1>;

template <typename T1>
using t29 = typename std::__map_iterator<T1>::iterator_category;

template <typename T1> using t30 = typename std::__map_iterator<T1>::value_type;

template <typename T1>
using t31 = typename std::__map_iterator<T1>::difference_type;

template <typename T1> using t32 = typename std::__map_iterator<T1>::reference;

template <typename T1> using t33 = typename std::__map_iterator<T1>::pointer;

template <typename T1> std::__map_iterator<T1> f136() {
  return std::__map_iterator<T1>();
}

template <typename T1> std::__map_iterator<T1> f137(T1 a0) {
  return std::__map_iterator<T1>(std::move(a0));
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

template <typename T1, typename T2, typename T3>
using t34 = std::__tree_const_iterator<T1, T2, T3>;

template <typename T1, typename T2, typename T3>
using t35 = typename std::__tree_const_iterator<T1, T2, T3>::iterator_category;

template <typename T1, typename T2, typename T3>
using t36 = typename std::__tree_const_iterator<T1, T2, T3>::value_type;

template <typename T1, typename T2, typename T3>
using t37 = typename std::__tree_const_iterator<T1, T2, T3>::difference_type;

template <typename T1, typename T2, typename T3>
using t38 = typename std::__tree_const_iterator<T1, T2, T3>::reference;

template <typename T1, typename T2, typename T3>
using t39 = typename std::__tree_const_iterator<T1, T2, T3>::pointer;

template <typename T1, typename T2, typename T3>
std::__tree_const_iterator<T1, T2, T3> f146() {
  return std::__tree_const_iterator<T1, T2, T3>();
}

template <typename T1, typename T2, typename T3>
const T1 &f147(const std::__tree_const_iterator<T1, T2, T3> &a0) {
  return a0.operator*();
}

template <typename T1, typename T2, typename T3>
typename std::__tree_const_iterator<T1, T2, T3>::pointer
f148(const std::__tree_const_iterator<T1, T2, T3> &a0) {
  return a0.operator->();
}

template <typename T1, typename T2, typename T3>
std::__tree_const_iterator<T1, T2, T3> &
f149(std::__tree_const_iterator<T1, T2, T3> &a0) {
  return a0.operator++();
}

template <typename T1, typename T2, typename T3>
std::__tree_const_iterator<T1, T2, T3>
f150(std::__tree_const_iterator<T1, T2, T3> &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1, typename T2, typename T3>
std::__tree_const_iterator<T1, T2, T3> &
f151(std::__tree_const_iterator<T1, T2, T3> &a0) {
  return a0.operator--();
}

template <typename T1, typename T2, typename T3>
std::__tree_const_iterator<T1, T2, T3>
f152(std::__tree_const_iterator<T1, T2, T3> &a0, int a1) {
  return a0.operator--(a1);
}

template <typename T1, typename T2, typename T3>
bool f153(const std::__tree_const_iterator<T1, T2, T3> &a0,
          const std::__tree_const_iterator<T1, T2, T3> &a1) {
  return operator!=(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f154(const std::__tree_const_iterator<T1, T2, T3> &a0,
          const std::__tree_const_iterator<T1, T2, T3> &a1) {
  return operator==(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::map<T1, T2, T3, T4> f155(const T3 &a0) {
  return std::map<T1, T2, T3, T4>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::map<T1, T2, T3, T4> f156(std::initializer_list<std::pair<const T1, T2>> a0,
                              const T3 &a1) {
  return std::map<T1, T2, T3, T4>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::map<T1, T2, T3, T4> f157(T5 a0, T5 a1, const T3 &a2) {
  return std::map<T1, T2, T3, T4>(std::move(a0), std::move(a1), a2);
}
