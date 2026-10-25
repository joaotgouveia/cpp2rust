#include <initializer_list>
#include <unordered_map>
#include <utility>

template <typename T1> using t42 = std::__hash_const_iterator<T1>;

template <typename T1>
using t43 = typename std::__hash_const_iterator<T1>::iterator_category;

template <typename T1>
using t44 = typename std::__hash_const_iterator<T1>::value_type;

template <typename T1>
using t45 = typename std::__hash_const_iterator<T1>::difference_type;

template <typename T1>
using t46 = typename std::__hash_const_iterator<T1>::reference;

template <typename T1>
using t47 = typename std::__hash_const_iterator<T1>::pointer;

template <typename T1> std::__hash_const_iterator<T1> f111() {
  return std::__hash_const_iterator<T1>();
}

template <typename T1>
const typename std::__hash_const_iterator<T1>::value_type &
f112(const std::__hash_const_iterator<T1> &a0) {
  return a0.operator*();
}

template <typename T1>
typename std::__hash_const_iterator<T1>::pointer
f113(const std::__hash_const_iterator<T1> &a0) {
  return a0.operator->();
}

template <typename T1>
std::__hash_const_iterator<T1> &f114(std::__hash_const_iterator<T1> &a0) {
  return a0.operator++();
}

template <typename T1>
std::__hash_const_iterator<T1> f115(std::__hash_const_iterator<T1> &a0,
                                    int a1) {
  return a0.operator++(a1);
}

template <typename T1>
bool f116(const std::__hash_const_iterator<T1> &a0,
          const std::__hash_const_iterator<T1> &a1) {
  return operator!=(a0, a1);
}

template <typename T1>
bool f117(const std::__hash_const_iterator<T1> &a0,
          const std::__hash_const_iterator<T1> &a1) {
  return operator==(a0, a1);
}

template <typename T1> using t48 = std::__hash_const_local_iterator<T1>;

template <typename T1>
using t49 = typename std::__hash_const_local_iterator<T1>::iterator_category;

template <typename T1>
using t50 = typename std::__hash_const_local_iterator<T1>::value_type;

template <typename T1>
using t51 = typename std::__hash_const_local_iterator<T1>::difference_type;

template <typename T1>
using t52 = typename std::__hash_const_local_iterator<T1>::reference;

template <typename T1>
using t53 = typename std::__hash_const_local_iterator<T1>::pointer;

template <typename T1> std::__hash_const_local_iterator<T1> f118() {
  return std::__hash_const_local_iterator<T1>();
}

template <typename T1>
const typename std::__hash_const_local_iterator<T1>::value_type &
f119(const std::__hash_const_local_iterator<T1> &a0) {
  return a0.operator*();
}

template <typename T1>
typename std::__hash_const_local_iterator<T1>::pointer
f120(const std::__hash_const_local_iterator<T1> &a0) {
  return a0.operator->();
}

template <typename T1>
std::__hash_const_local_iterator<T1> &
f121(std::__hash_const_local_iterator<T1> &a0) {
  return a0.operator++();
}

template <typename T1>
std::__hash_const_local_iterator<T1>
f122(std::__hash_const_local_iterator<T1> &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1>
bool f123(const std::__hash_const_local_iterator<T1> &a0,
          const std::__hash_const_local_iterator<T1> &a1) {
  return operator!=(a0, a1);
}

template <typename T1>
bool f124(const std::__hash_const_local_iterator<T1> &a0,
          const std::__hash_const_local_iterator<T1> &a1) {
  return operator==(a0, a1);
}

template <typename T1> using t54 = std::__hash_map_const_iterator<T1>;

template <typename T1>
using t55 = typename std::__hash_map_const_iterator<T1>::iterator_category;

template <typename T1>
using t56 = typename std::__hash_map_const_iterator<T1>::value_type;

template <typename T1>
using t57 = typename std::__hash_map_const_iterator<T1>::difference_type;

template <typename T1>
using t58 = typename std::__hash_map_const_iterator<T1>::reference;

template <typename T1>
using t59 = typename std::__hash_map_const_iterator<T1>::pointer;

template <typename T1> std::__hash_map_const_iterator<T1> f125() {
  return std::__hash_map_const_iterator<T1>();
}

template <typename T1> std::__hash_map_const_iterator<T1> f126(T1 a0) {
  return std::__hash_map_const_iterator<T1>(std::move(a0));
}

template <typename T1>
std::__hash_map_const_iterator<T1>
f127(std::__hash_map_iterator<typename T1::__non_const_iterator> a0) {
  return std::__hash_map_const_iterator<T1>(std::move(a0));
}

template <typename T1>
const typename std::__hash_map_const_iterator<T1>::value_type &
f128(const std::__hash_map_const_iterator<T1> &a0) {
  return a0.operator*();
}

template <typename T1>
typename std::__hash_map_const_iterator<T1>::pointer
f129(const std::__hash_map_const_iterator<T1> &a0) {
  return a0.operator->();
}

template <typename T1>
std::__hash_map_const_iterator<T1> &
f130(std::__hash_map_const_iterator<T1> &a0) {
  return a0.operator++();
}

template <typename T1>
std::__hash_map_const_iterator<T1> f131(std::__hash_map_const_iterator<T1> &a0,
                                        int a1) {
  return a0.operator++(a1);
}

template <typename T1>
bool f132(const std::__hash_map_const_iterator<T1> &a0,
          const std::__hash_map_const_iterator<T1> &a1) {
  return operator==(a0, a1);
}

template <typename T1> using t60 = std::__hash_map_iterator<T1>;

template <typename T1>
using t61 = typename std::__hash_map_iterator<T1>::iterator_category;

template <typename T1>
using t62 = typename std::__hash_map_iterator<T1>::value_type;

template <typename T1>
using t63 = typename std::__hash_map_iterator<T1>::difference_type;

template <typename T1>
using t64 = typename std::__hash_map_iterator<T1>::reference;

template <typename T1>
using t65 = typename std::__hash_map_iterator<T1>::pointer;

template <typename T1> std::__hash_map_iterator<T1> f133() {
  return std::__hash_map_iterator<T1>();
}

template <typename T1> std::__hash_map_iterator<T1> f134(T1 a0) {
  return std::__hash_map_iterator<T1>(std::move(a0));
}

template <typename T1>
typename std::__hash_map_iterator<T1>::value_type &
f135(const std::__hash_map_iterator<T1> &a0) {
  return a0.operator*();
}

template <typename T1>
typename std::__hash_map_iterator<T1>::pointer
f136(const std::__hash_map_iterator<T1> &a0) {
  return a0.operator->();
}

template <typename T1>
std::__hash_map_iterator<T1> &f137(std::__hash_map_iterator<T1> &a0) {
  return a0.operator++();
}

template <typename T1>
std::__hash_map_iterator<T1> f138(std::__hash_map_iterator<T1> &a0, int a1) {
  return a0.operator++(a1);
}

template <typename T1>
bool f139(const std::__hash_map_iterator<T1> &a0,
          const std::__hash_map_iterator<T1> &a1) {
  return operator==(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_map<T1, T2, T3, T4, T5>
f140(typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a0,
     const T3 &a1, const T4 &a2) {
  return std::unordered_map<T1, T2, T3, T4, T5>(std::move(a0), a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_map<T1, T2, T3, T4, T5>
f141(typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a0,
     const T3 &a1, const T4 &a2, const T5 &a3) {
  return std::unordered_map<T1, T2, T3, T4, T5>(std::move(a0), a1, a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_map<T1, T2, T3, T4, T5> f142(const T5 &a0) {
  return std::unordered_map<T1, T2, T3, T4, T5>(a0);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_map<T1, T2, T3, T4, T5>
f143(const std::unordered_map<T1, T2, T3, T4, T5> &a0, const T5 &a1) {
  return std::unordered_map<T1, T2, T3, T4, T5>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_map<T1, T2, T3, T4, T5>
f144(std::unordered_map<T1, T2, T3, T4, T5> &&a0, const T5 &a1) {
  return std::unordered_map<T1, T2, T3, T4, T5>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_map<T1, T2, T3, T4, T5>
f145(std::initializer_list<std::pair<const T1, T2>> a0) {
  return std::unordered_map<T1, T2, T3, T4, T5>(std::move(a0));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_map<T1, T2, T3, T4, T5>
f146(std::initializer_list<std::pair<const T1, T2>> a0,
     typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a1,
     const T3 &a2, const T4 &a3) {
  return std::unordered_map<T1, T2, T3, T4, T5>(std::move(a0), std::move(a1),
                                                a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_map<T1, T2, T3, T4, T5>
f147(std::initializer_list<std::pair<const T1, T2>> a0,
     typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a1,
     const T3 &a2, const T4 &a3, const T5 &a4) {
  return std::unordered_map<T1, T2, T3, T4, T5>(std::move(a0), std::move(a1),
                                                a2, a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_map<T1, T2, T3, T4, T5>
f148(typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a0,
     const T5 &a1) {
  return std::unordered_map<T1, T2, T3, T4, T5>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_map<T1, T2, T3, T4, T5>
f149(typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a0,
     const T3 &a1, const T5 &a2) {
  return std::unordered_map<T1, T2, T3, T4, T5>(std::move(a0), a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_map<T1, T2, T3, T4, T5>
f150(std::initializer_list<std::pair<const T1, T2>> a0,
     typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a1,
     const T5 &a2) {
  return std::unordered_map<T1, T2, T3, T4, T5>(std::move(a0), std::move(a1),
                                                a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_map<T1, T2, T3, T4, T5>
f151(std::initializer_list<std::pair<const T1, T2>> a0,
     typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a1,
     const T3 &a2, const T5 &a3) {
  return std::unordered_map<T1, T2, T3, T4, T5>(std::move(a0), std::move(a1),
                                                a2, a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::unordered_map<T1, T2, T3, T4, T5> &
f152(std::unordered_map<T1, T2, T3, T4, T5> &a0,
     std::initializer_list<std::pair<const T1, T2>> a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
T5 f153(const std::unordered_map<T1, T2, T3, T4, T5> &a0) {
  return a0.get_allocator();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::pair<typename std::unordered_map<T1, T2, T3, T4, T5>::iterator, bool>
f154(std::unordered_map<T1, T2, T3, T4, T5> &a0,
     const std::pair<const T1, T2> &a1) {
  return a0.insert(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::iterator
f155(std::unordered_map<T1, T2, T3, T4, T5> &a0,
     typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator a1,
     const std::pair<const T1, T2> &a2) {
  return a0.insert(std::move(a1), a2);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
void f156(std::unordered_map<T1, T2, T3, T4, T5> &a0,
          std::initializer_list<std::pair<const T1, T2>> a1) {
  return a0.insert(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::pair<typename std::unordered_map<T1, T2, T3, T4, T5>::iterator, bool>
f157(std::unordered_map<T1, T2, T3, T4, T5> &a0, std::pair<const T1, T2> &&a1) {
  return a0.insert(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::iterator
f158(std::unordered_map<T1, T2, T3, T4, T5> &a0,
     typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator a1,
     std::pair<const T1, T2> &&a2) {
  return a0.insert(std::move(a1), std::move(a2));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::size_type
f159(std::unordered_map<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.erase(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
T3 f160(const std::unordered_map<T1, T2, T3, T4, T5> &a0) {
  return a0.hash_function();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
T4 f161(const std::unordered_map<T1, T2, T3, T4, T5> &a0) {
  return a0.key_eq();
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::iterator
f162(std::unordered_map<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator
f163(const std::unordered_map<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.find(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::size_type
f164(const std::unordered_map<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.count(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::pair<typename std::unordered_map<T1, T2, T3, T4, T5>::iterator,
          typename std::unordered_map<T1, T2, T3, T4, T5>::iterator>
f165(std::unordered_map<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
std::pair<typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator,
          typename std::unordered_map<T1, T2, T3, T4, T5>::const_iterator>
f166(const std::unordered_map<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.equal_range(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
T2 &f167(std::unordered_map<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
T2 &f168(std::unordered_map<T1, T2, T3, T4, T5> &a0, T1 &&a1) {
  return a0.operator[](std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
T2 &f169(std::unordered_map<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.at(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
const T2 &f170(const std::unordered_map<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.at(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5>
typename std::unordered_map<T1, T2, T3, T4, T5>::size_type
f171(const std::unordered_map<T1, T2, T3, T4, T5> &a0, const T1 &a1) {
  return a0.bucket(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::unordered_map<T1, T2, T3, T4, T5> f172(T6 a0, T6 a1) {
  return std::unordered_map<T1, T2, T3, T4, T5>(std::move(a0), std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::unordered_map<T1, T2, T3, T4, T5>
f173(T6 a0, T6 a1,
     typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a2,
     const T3 &a3, const T4 &a4) {
  return std::unordered_map<T1, T2, T3, T4, T5>(std::move(a0), std::move(a1),
                                                std::move(a2), a3, a4);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::unordered_map<T1, T2, T3, T4, T5>
f174(T6 a0, T6 a1,
     typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a2,
     const T3 &a3, const T4 &a4, const T5 &a5) {
  return std::unordered_map<T1, T2, T3, T4, T5>(std::move(a0), std::move(a1),
                                                std::move(a2), a3, a4, a5);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::unordered_map<T1, T2, T3, T4, T5>
f175(T6 a0, T6 a1,
     typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a2,
     const T5 &a3) {
  return std::unordered_map<T1, T2, T3, T4, T5>(std::move(a0), std::move(a1),
                                                std::move(a2), a3);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
std::unordered_map<T1, T2, T3, T4, T5>
f176(T6 a0, T6 a1,
     typename std::unordered_map<T1, T2, T3, T4, T5>::size_type a2,
     const T3 &a3, const T5 &a4) {
  return std::unordered_map<T1, T2, T3, T4, T5>(std::move(a0), std::move(a1),
                                                std::move(a2), a3, a4);
}
