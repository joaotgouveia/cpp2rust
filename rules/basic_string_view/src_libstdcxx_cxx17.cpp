#include <algorithm>
#include <string_view>
#include <utility>

template <typename T1, typename T2> using t1 = std::basic_string_view<T1, T2>;

template <typename T1, typename T2>
using t2 = typename std::basic_string_view<T1, T2>::traits_type;

template <typename T1, typename T2>
using t3 = typename std::basic_string_view<T1, T2>::value_type;

template <typename T1, typename T2>
using t4 = typename std::basic_string_view<T1, T2>::pointer;

template <typename T1, typename T2>
using t5 = typename std::basic_string_view<T1, T2>::const_pointer;

template <typename T1, typename T2>
using t6 = typename std::basic_string_view<T1, T2>::reference;

template <typename T1, typename T2>
using t7 = typename std::basic_string_view<T1, T2>::const_reference;

template <typename T1, typename T2>
using t8 = typename std::basic_string_view<T1, T2>::const_iterator;

template <typename T1, typename T2>
using t9 = typename std::basic_string_view<T1, T2>::iterator;

template <typename T1, typename T2>
using t10 = typename std::basic_string_view<T1, T2>::const_reverse_iterator;

template <typename T1, typename T2>
using t11 = typename std::basic_string_view<T1, T2>::reverse_iterator;

template <typename T1, typename T2>
using t12 = typename std::basic_string_view<T1, T2>::size_type;

template <typename T1, typename T2>
using t13 = typename std::basic_string_view<T1, T2>::difference_type;

template <typename T1, typename T2> std::basic_string_view<T1, T2> f11() {
  return std::basic_string_view<T1, T2>();
}

template <typename T1, typename T2>
std::basic_string_view<T1, T2> f12(const std::basic_string_view<T1, T2> &a0) {
  return std::basic_string_view<T1, T2>(a0);
}

template <typename T1, typename T2>
std::basic_string_view<T1, T2> f13(const T1 *a0) {
  return std::basic_string_view<T1, T2>(a0);
}

template <typename T1, typename T2>
std::basic_string_view<T1, T2> f14(const T1 *a0, std::size_t a1) {
  return std::basic_string_view<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::basic_string_view<T1, T2> &f15(std::basic_string_view<T1, T2> &a0,
                                    const std::basic_string_view<T1, T2> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
const T1 *f16(const std::basic_string_view<T1, T2> &a0) {
  return a0.begin();
}

template <typename T1, typename T2>
const T1 *f17(const std::basic_string_view<T1, T2> &a0) {
  return a0.end();
}

template <typename T1, typename T2>
const T1 *f18(const std::basic_string_view<T1, T2> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2>
const T1 *f19(const std::basic_string_view<T1, T2> &a0) {
  return a0.cend();
}

template <typename T1, typename T2>
std::reverse_iterator<const T1 *>
f20(const std::basic_string_view<T1, T2> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2>
std::reverse_iterator<const T1 *>
f21(const std::basic_string_view<T1, T2> &a0) {
  return a0.rend();
}

template <typename T1, typename T2>
std::reverse_iterator<const T1 *>
f22(const std::basic_string_view<T1, T2> &a0) {
  return a0.crbegin();
}

template <typename T1, typename T2>
std::reverse_iterator<const T1 *>
f23(const std::basic_string_view<T1, T2> &a0) {
  return a0.crend();
}

template <typename T1, typename T2>
std::size_t f24(const std::basic_string_view<T1, T2> &a0) {
  return a0.size();
}

template <typename T1, typename T2>
std::size_t f25(const std::basic_string_view<T1, T2> &a0) {
  return a0.length();
}

template <typename T1, typename T2>
std::size_t f26(const std::basic_string_view<T1, T2> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2>
bool f27(const std::basic_string_view<T1, T2> &a0) {
  return a0.empty();
}

template <typename T1, typename T2>
const T1 &f28(const std::basic_string_view<T1, T2> &a0, std::size_t a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2>
const T1 &f29(const std::basic_string_view<T1, T2> &a0, std::size_t a1) {
  return a0.at(a1);
}

template <typename T1, typename T2>
const T1 &f30(const std::basic_string_view<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
const T1 &f31(const std::basic_string_view<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
const T1 *f32(const std::basic_string_view<T1, T2> &a0) {
  return a0.data();
}

template <typename T1, typename T2>
void f33(std::basic_string_view<T1, T2> &a0, std::size_t a1) {
  return a0.remove_prefix(a1);
}

template <typename T1, typename T2>
void f34(std::basic_string_view<T1, T2> &a0, std::size_t a1) {
  return a0.remove_suffix(a1);
}

template <typename T1, typename T2>
void f35(std::basic_string_view<T1, T2> &a0,
         std::basic_string_view<T1, T2> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2>
std::size_t f36(const std::basic_string_view<T1, T2> &a0, T1 *a1,
                std::size_t a2, std::size_t a3) {
  return a0.copy(a1, a2, a3);
}

template <typename T1, typename T2>
std::basic_string_view<T1, T2> f37(const std::basic_string_view<T1, T2> &a0,
                                   std::size_t a1, std::size_t a2) {
  return a0.substr(a1, a2);
}

template <typename T1, typename T2>
int f38(const std::basic_string_view<T1, T2> &a0,
        std::basic_string_view<T1, T2> a1) {
  return a0.compare(a1);
}

template <typename T1, typename T2>
int f39(const std::basic_string_view<T1, T2> &a0, std::size_t a1,
        std::size_t a2, std::basic_string_view<T1, T2> a3) {
  return a0.compare(a1, a2, a3);
}

template <typename T1, typename T2>
int f40(const std::basic_string_view<T1, T2> &a0, std::size_t a1,
        std::size_t a2, std::basic_string_view<T1, T2> a3, std::size_t a4,
        std::size_t a5) {
  return a0.compare(a1, a2, a3, a4, a5);
}

template <typename T1, typename T2>
int f41(const std::basic_string_view<T1, T2> &a0, const T1 *a1) {
  return a0.compare(a1);
}

template <typename T1, typename T2>
int f42(const std::basic_string_view<T1, T2> &a0, std::size_t a1,
        std::size_t a2, const T1 *a3) {
  return a0.compare(a1, a2, a3);
}

template <typename T1, typename T2>
int f43(const std::basic_string_view<T1, T2> &a0, std::size_t a1,
        std::size_t a2, const T1 *a3, std::size_t a4) {
  return a0.compare(a1, a2, a3, a4);
}

template <typename T1, typename T2>
std::size_t f44(const std::basic_string_view<T1, T2> &a0,
                std::basic_string_view<T1, T2> a1, std::size_t a2) {
  return a0.find(a1, a2);
}

template <typename T1, typename T2>
std::size_t f45(const std::basic_string_view<T1, T2> &a0, T1 a1,
                std::size_t a2) {
  return a0.find(a1, a2);
}

template <typename T1, typename T2>
std::size_t f46(const std::basic_string_view<T1, T2> &a0, const T1 *a1,
                std::size_t a2, std::size_t a3) {
  return a0.find(a1, a2, a3);
}

template <typename T1, typename T2>
std::size_t f47(const std::basic_string_view<T1, T2> &a0, const T1 *a1,
                std::size_t a2) {
  return a0.find(a1, a2);
}

template <typename T1, typename T2>
std::size_t f48(const std::basic_string_view<T1, T2> &a0,
                std::basic_string_view<T1, T2> a1, std::size_t a2) {
  return a0.rfind(a1, a2);
}

template <typename T1, typename T2>
std::size_t f49(const std::basic_string_view<T1, T2> &a0, T1 a1,
                std::size_t a2) {
  return a0.rfind(a1, a2);
}

template <typename T1, typename T2>
std::size_t f50(const std::basic_string_view<T1, T2> &a0, const T1 *a1,
                std::size_t a2, std::size_t a3) {
  return a0.rfind(a1, a2, a3);
}

template <typename T1, typename T2>
std::size_t f51(const std::basic_string_view<T1, T2> &a0, const T1 *a1,
                std::size_t a2) {
  return a0.rfind(a1, a2);
}

template <typename T1, typename T2>
std::size_t f52(const std::basic_string_view<T1, T2> &a0,
                std::basic_string_view<T1, T2> a1, std::size_t a2) {
  return a0.find_first_of(a1, a2);
}

template <typename T1, typename T2>
std::size_t f53(const std::basic_string_view<T1, T2> &a0, T1 a1,
                std::size_t a2) {
  return a0.find_first_of(a1, a2);
}

template <typename T1, typename T2>
std::size_t f54(const std::basic_string_view<T1, T2> &a0, const T1 *a1,
                std::size_t a2, std::size_t a3) {
  return a0.find_first_of(a1, a2, a3);
}

template <typename T1, typename T2>
std::size_t f55(const std::basic_string_view<T1, T2> &a0, const T1 *a1,
                std::size_t a2) {
  return a0.find_first_of(a1, a2);
}

template <typename T1, typename T2>
std::size_t f56(const std::basic_string_view<T1, T2> &a0,
                std::basic_string_view<T1, T2> a1, std::size_t a2) {
  return a0.find_last_of(a1, a2);
}

template <typename T1, typename T2>
std::size_t f57(const std::basic_string_view<T1, T2> &a0, T1 a1,
                std::size_t a2) {
  return a0.find_last_of(a1, a2);
}

template <typename T1, typename T2>
std::size_t f58(const std::basic_string_view<T1, T2> &a0, const T1 *a1,
                std::size_t a2, std::size_t a3) {
  return a0.find_last_of(a1, a2, a3);
}

template <typename T1, typename T2>
std::size_t f59(const std::basic_string_view<T1, T2> &a0, const T1 *a1,
                std::size_t a2) {
  return a0.find_last_of(a1, a2);
}

template <typename T1, typename T2>
std::size_t f60(const std::basic_string_view<T1, T2> &a0,
                std::basic_string_view<T1, T2> a1, std::size_t a2) {
  return a0.find_first_not_of(a1, a2);
}

template <typename T1, typename T2>
std::size_t f61(const std::basic_string_view<T1, T2> &a0, T1 a1,
                std::size_t a2) {
  return a0.find_first_not_of(a1, a2);
}

template <typename T1, typename T2>
std::size_t f62(const std::basic_string_view<T1, T2> &a0, const T1 *a1,
                std::size_t a2, std::size_t a3) {
  return a0.find_first_not_of(a1, a2, a3);
}

template <typename T1, typename T2>
std::size_t f63(const std::basic_string_view<T1, T2> &a0, const T1 *a1,
                std::size_t a2) {
  return a0.find_first_not_of(a1, a2);
}

template <typename T1, typename T2>
std::size_t f64(const std::basic_string_view<T1, T2> &a0,
                std::basic_string_view<T1, T2> a1, std::size_t a2) {
  return a0.find_last_not_of(a1, a2);
}

template <typename T1, typename T2>
std::size_t f65(const std::basic_string_view<T1, T2> &a0, T1 a1,
                std::size_t a2) {
  return a0.find_last_not_of(a1, a2);
}

template <typename T1, typename T2>
std::size_t f66(const std::basic_string_view<T1, T2> &a0, const T1 *a1,
                std::size_t a2, std::size_t a3) {
  return a0.find_last_not_of(a1, a2, a3);
}

template <typename T1, typename T2>
std::size_t f67(const std::basic_string_view<T1, T2> &a0, const T1 *a1,
                std::size_t a2) {
  return a0.find_last_not_of(a1, a2);
}

template <typename T1, typename T2> std::size_t f68() {
  return std::basic_string_view<T1, T2>::npos;
}
