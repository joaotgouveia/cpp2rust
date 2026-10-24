#include <initializer_list>
#include <unordered_map>
#include <utility>

template <typename T1> using t36 = std::__hash_map_const_iterator<T1>;

template <typename T1>
using t37 = typename std::__hash_map_const_iterator<T1>::iterator_category;

template <typename T1>
using t38 = typename std::__hash_map_const_iterator<T1>::value_type;

template <typename T1>
using t39 = typename std::__hash_map_const_iterator<T1>::difference_type;

template <typename T1>
using t40 = typename std::__hash_map_const_iterator<T1>::reference;

template <typename T1>
using t41 = typename std::__hash_map_const_iterator<T1>::pointer;

template <typename T1> std::__hash_map_const_iterator<T1> f111() {
  return std::__hash_map_const_iterator<T1>();
}

template <typename T1> std::__hash_map_const_iterator<T1> f112(T1 a0) {
  return std::__hash_map_const_iterator<T1>(a0);
}

template <typename T1>
const typename std::__hash_map_const_iterator<T1>::value_type &
f113(const std::__hash_map_const_iterator<T1> &a0) {
  return a0.operator*();
}

template <typename T1>
typename std::__hash_map_const_iterator<T1>::pointer
f114(const std::__hash_map_const_iterator<T1> &a0) {
  return a0.operator->();
}

template <typename T1>
std::__hash_map_const_iterator<T1> &
f115(std::__hash_map_const_iterator<T1> &a0) {
  return a0.operator++();
}

template <typename T1>
std::__hash_map_const_iterator<T1> f116(std::__hash_map_const_iterator<T1> &a0,
                                        int a1) {
  return a0.operator++(a1);
}

template <typename T1>
bool f117(const std::__hash_map_const_iterator<T1> &a0,
          const std::__hash_map_const_iterator<T1> &a1) {
  return operator==(a0, a1);
}

template <typename T1> using t42 = std::__hash_map_iterator<T1>;

template <typename T1>
using t43 = typename std::__hash_map_iterator<T1>::iterator_category;

template <typename T1>
using t44 = typename std::__hash_map_iterator<T1>::value_type;

template <typename T1>
using t45 = typename std::__hash_map_iterator<T1>::difference_type;

template <typename T1>
using t46 = typename std::__hash_map_iterator<T1>::reference;

template <typename T1>
using t47 = typename std::__hash_map_iterator<T1>::pointer;

template <typename T1> std::__hash_map_iterator<T1> f118() {
  return std::__hash_map_iterator<T1>();
}

template <typename T1> std::__hash_map_iterator<T1> f119(T1 a0) {
  return std::__hash_map_iterator<T1>(a0);
}

template <typename T1>
typename std::__hash_map_iterator<T1>::value_type &
f120(const std::__hash_map_iterator<T1> &a0) {
  return a0.operator*();
}

template <typename T1>
typename std::__hash_map_iterator<T1>::pointer
f121(const std::__hash_map_iterator<T1> &a0) {
  return a0.operator->();
}

template <typename T1>
std::__hash_map_iterator<T1> &f122(std::__hash_map_iterator<T1> &a0) {
  return a0.operator++();
}

template <typename T1>
std::__hash_map_iterator<T1> f123(std::__hash_map_iterator<T1> &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1>
bool f124(const std::__hash_map_iterator<T1> &a0,
          const std::__hash_map_iterator<T1> &a1) {
  return operator==(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_map<T1, T2, T3, T4, T5>
f125(typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a0,
     const T3 &a1, const T4 &a2) {
  return std::unordered_map<T1, T2, T3, T4, T5>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_map<T1, T2, T3, T4, T5>
f126(typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a0,
     const T3 &a1, const T4 &a2, const T5 &a3) {
  return std::unordered_map<T1, T2, T3, T4, T5>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_map<T1, T2, T3, T4, T5> f127(const T5 &a0) {
  return std::unordered_map<T1, T2, T3, T4, T5>(a0);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_map<T1, T2, T3, T4, T5>
f128(const std::unordered_map<T1, T2, T3, T4, T5> &a0, const T5 &a1) {
  return std::unordered_map<T1, T2, T3, T4, T5>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_map<T1, T2, T3, T4, T5>
f129(std::unordered_map<T1, T2, T3, T4, T5> &&a0, const T5 &a1) {
  return std::unordered_map<T1, T2, T3, T4, T5>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_map<T1, T2, T3, T4, T5>
f130(std::initializer_list<std::pair<const T1, T2>> a0) {
  return std::unordered_map<T1, T2, T3, T4, T5>(a0);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_map<T1, T2, T3, T4, T5>
f131(std::initializer_list<std::pair<const T1, T2>> a0,
     typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a1,
     const T3 &a2, const T4 &a3) {
  return std::unordered_map<T1, T2, T3, T4, T5>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_map<T1, T2, T3, T4, T5>
f132(std::initializer_list<std::pair<const T1, T2>> a0,
     typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a1,
     const T3 &a2, const T4 &a3, const T5 &a4) {
  return std::unordered_map<T1, T2, T3, T4, T5>(a0, a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_map<T1, T2, T3, T4, T5>
f133(typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a0,
     const T5 &a1) {
  return std::unordered_map<T1, T2, T3, T4, T5>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_map<T1, T2, T3, T4, T5>
f134(typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a0,
     const T3 &a1, const T5 &a2) {
  return std::unordered_map<T1, T2, T3, T4, T5>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_map<T1, T2, T3, T4, T5>
f135(std::initializer_list<std::pair<const T1, T2>> a0,
     typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a1,
     const T5 &a2) {
  return std::unordered_map<T1, T2, T3, T4, T5>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_map<T1, T2, T3, T4, T5>
f136(std::initializer_list<std::pair<const T1, T2>> a0,
     typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a1,
     const T3 &a2, const T5 &a3) {
  return std::unordered_map<T1, T2, T3, T4, T5>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_map<T1, T2, T3, T4, T5> &
f137(std::unordered_map<T1, T2, T3, T4, T5> &a0,
     std::initializer_list<std::pair<const T1, T2>> a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
T5 f138(const std::unordered_map<T1, T2, T3, T4, T5> &a0) {
  return a0.get_allocator();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::pair<typename std::unordered_map<T1, T2, T3, T4, T5>::iterator, bool>
f139(std::unordered_map<T1, T2, T3, T4, T5> &a0,
     const std::pair<const T1, T2> &a1) {
  return a0.insert(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::iterator
f140(std::unordered_map<T1, T2, T3, T4, T5> &a0,
     typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator a1,
     const std::pair<const T1, T2> &a2) {
  return a0.insert(a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f141(std::unordered_map<T1, T2, T3, T4, T5> &a0,
          std::initializer_list<std::pair<const T1, T2>> a1) {
  return a0.insert(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::pair<typename std::unordered_map<T1, T2, T3, T4, T5>::iterator, bool>
f142(std::unordered_map<T1, T2, T3, T4, T5> &a0, std::pair<const T1, T2> &&a1) {
  return a0.insert(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::iterator
f143(std::unordered_map<T1, T2, T3, T4, T5> &a0,
     typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator a1,
     std::pair<const T1, T2> &&a2) {
  return a0.insert(a1, std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::size_type
f144(std::unordered_map<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
T3 f145(const std::unordered_map<T1, T2, T3, T4, T5> &a0) {
  return a0.hash_function();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
T4 f146(const std::unordered_map<T1, T2, T3, T4, T5> &a0) {
  return a0.key_eq();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::iterator
f147(std::unordered_map<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator
f148(const std::unordered_map<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::size_type
f149(const std::unordered_map<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.count(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::pair<typename std::unordered_map<T1, T2, T3, T4, T5>::iterator,
          typename std::unordered_map<T1, T2, T3, T4, T5>::iterator>
f150(std::unordered_map<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::pair<typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator,
          typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator>
f151(const std::unordered_map<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
T2 &f152(std::unordered_map<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
T2 &f153(std::unordered_map<T1, T2, T3, T4, T5> &a0, T1 &&a1) {
  return a0.operator[](std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
T2 &f154(std::unordered_map<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.at(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
const T2 &f155(const std::unordered_map<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.at(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::size_type
f156(const std::unordered_map<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.bucket(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::unordered_map<T1, T2, T3, T4, T5> f157(T6 a0, T6 a1) {
  return std::unordered_map<T1, T2, T3, T4, T5>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::unordered_map<T1, T2, T3, T4, T5>
f158(T6 a0, T6 a1,
     typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a2,
     const T3 &a3, const T4 &a4) {
  return std::unordered_map<T1, T2, T3, T4, T5>(a0, a1, a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::unordered_map<T1, T2, T3, T4, T5>
f159(T6 a0, T6 a1,
     typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a2,
     const T3 &a3, const T4 &a4, const T5 &a5) {
  return std::unordered_map<T1, T2, T3, T4, T5>(a0, a1, a2, a3, a4, a5);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::unordered_map<T1, T2, T3, T4, T5>
f160(T6 a0, T6 a1,
     typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a2,
     const T5 &a3) {
  return std::unordered_map<T1, T2, T3, T4, T5>(a0, a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::unordered_map<T1, T2, T3, T4, T5>
f161(T6 a0, T6 a1,
     typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a2,
     const T3 &a3, const T5 &a4) {
  return std::unordered_map<T1, T2, T3, T4, T5>(a0, a1, a2, a3, a4);
}
