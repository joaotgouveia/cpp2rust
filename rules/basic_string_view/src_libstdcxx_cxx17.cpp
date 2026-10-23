#include <string_view>
#include <utility>

template <typename T1, typename T2> using t3 = std::basic_string_view<T1, T2>;

template <typename T1, typename T2>
using t4 = typename std::basic_string_view<T1, T2>::traits_type;

template <typename T1, typename T2>
using t5 = typename std::basic_string_view<T1, T2>::value_type;

template <typename T1, typename T2>
using t6 = typename std::basic_string_view<T1, T2>::pointer;

template <typename T1, typename T2>
using t7 = typename std::basic_string_view<T1, T2>::const_pointer;

template <typename T1, typename T2>
using t8 = typename std::basic_string_view<T1, T2>::reference;

template <typename T1, typename T2>
using t9 = typename std::basic_string_view<T1, T2>::const_reference;

template <typename T1, typename T2>
using t10 = typename std::basic_string_view<T1, T2>::const_iterator;

template <typename T1, typename T2>
using t11 = typename std::basic_string_view<T1, T2>::iterator;

template <typename T1, typename T2>
using t12 = typename std::basic_string_view<T1, T2>::const_reverse_iterator;

template <typename T1, typename T2>
using t13 = typename std::basic_string_view<T1, T2>::reverse_iterator;

template <typename T1, typename T2>
using t14 = typename std::basic_string_view<T1, T2>::size_type;

template <typename T1, typename T2>
using t15 = typename std::basic_string_view<T1, T2>::difference_type;

template <typename T1, typename T2> std::basic_string_view<T1, T2> f5() {
  return std::basic_string_view<T1, T2>();
}

template <typename T1, typename T2>
std::basic_string_view<T1, T2> f6(const std::basic_string_view<T1, T2> &a0) {
  return std::basic_string_view<T1, T2>(a0);
}

template <typename T1, typename T2>
std::basic_string_view<T1, T2> f7(const T1 *a0) {
  return std::basic_string_view<T1, T2>(a0);
}

template <typename T1, typename T2>
std::basic_string_view<T1, T2>
f8(const T1 *a0, typename std::basic_string_view<T1, T2>::size_type a1) {
  return std::basic_string_view<T1, T2>(a0, a1);
}

template <typename T1, typename T2>
std::basic_string_view<T1, T2> &f9(std::basic_string_view<T1, T2> &a0,
                                   const std::basic_string_view<T1, T2> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2>
const typename std::basic_string_view<T1, T2>::value_type *
f10(const std::basic_string_view<T1, T2> &a0) {
  return a0.begin();
}

template <typename T1, typename T2>
const typename std::basic_string_view<T1, T2>::value_type *
f11(const std::basic_string_view<T1, T2> &a0) {
  return a0.end();
}

template <typename T1, typename T2>
const typename std::basic_string_view<T1, T2>::value_type *
f12(const std::basic_string_view<T1, T2> &a0) {
  return a0.cbegin();
}

template <typename T1, typename T2>
const typename std::basic_string_view<T1, T2>::value_type *
f13(const std::basic_string_view<T1, T2> &a0) {
  return a0.cend();
}

template <typename T1, typename T2>
typename std::basic_string_view<T1, T2>::const_reverse_iterator
f14(const std::basic_string_view<T1, T2> &a0) {
  return a0.rbegin();
}

template <typename T1, typename T2>
typename std::basic_string_view<T1, T2>::const_reverse_iterator
f15(const std::basic_string_view<T1, T2> &a0) {
  return a0.rend();
}

template <typename T1, typename T2>
typename std::basic_string_view<T1, T2>::const_reverse_iterator
f16(const std::basic_string_view<T1, T2> &a0) {
  return a0.crbegin();
}

template <typename T1, typename T2>
typename std::basic_string_view<T1, T2>::const_reverse_iterator
f17(const std::basic_string_view<T1, T2> &a0) {
  return a0.crend();
}

template <typename T1, typename T2>
typename std::basic_string_view<T1, T2>::size_type
f18(const std::basic_string_view<T1, T2> &a0) {
  return a0.size();
}

template <typename T1, typename T2>
typename std::basic_string_view<T1, T2>::size_type
f19(const std::basic_string_view<T1, T2> &a0) {
  return a0.length();
}

template <typename T1, typename T2>
typename std::basic_string_view<T1, T2>::size_type
f20(const std::basic_string_view<T1, T2> &a0) {
  return a0.max_size();
}

template <typename T1, typename T2>
bool f21(const std::basic_string_view<T1, T2> &a0) {
  return a0.empty();
}

template <typename T1, typename T2>
const typename std::basic_string_view<T1, T2>::value_type &
f22(const std::basic_string_view<T1, T2> &a0,
    typename std::basic_string_view<T1, T2>::size_type a1) {
  return a0.operator[](a1);
}

template <typename T1, typename T2>
const typename std::basic_string_view<T1, T2>::value_type &
f23(const std::basic_string_view<T1, T2> &a0,
    typename std::basic_string_view<T1, T2>::size_type a1) {
  return a0.at(a1);
}

template <typename T1, typename T2>
const typename std::basic_string_view<T1, T2>::value_type &
f24(const std::basic_string_view<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
const typename std::basic_string_view<T1, T2>::value_type &
f25(const std::basic_string_view<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
const typename std::basic_string_view<T1, T2>::value_type *
f26(const std::basic_string_view<T1, T2> &a0) {
  return a0.data();
}

template <typename T1, typename T2>
void f27(std::basic_string_view<T1, T2> &a0,
         typename std::basic_string_view<T1, T2>::size_type a1) {
  return a0.remove_prefix(a1);
}

template <typename T1, typename T2>
void f28(std::basic_string_view<T1, T2> &a0,
         typename std::basic_string_view<T1, T2>::size_type a1) {
  return a0.remove_suffix(a1);
}

template <typename T1, typename T2>
void f29(std::basic_string_view<T1, T2> &a0,
         std::basic_string_view<T1, T2> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2>
typename std::basic_string_view<T1, T2>::size_type
f30(const std::basic_string_view<T1, T2> &a0, T1 *a1,
    typename std::basic_string_view<T1, T2>::size_type a2,
    typename std::basic_string_view<T1, T2>::size_type a3) {
  return a0.copy(a1, a2, a3);
}

template <typename T1, typename T2>
std::basic_string_view<T1, T2>
f31(const std::basic_string_view<T1, T2> &a0,
    typename std::basic_string_view<T1, T2>::size_type a1,
    typename std::basic_string_view<T1, T2>::size_type a2) {
  return a0.substr(a1, a2);
}

template <typename T1, typename T2>
int f32(const std::basic_string_view<T1, T2> &a0,
        std::basic_string_view<T1, T2> a1) {
  return a0.compare(a1);
}

template <typename T1, typename T2>
int f33(const std::basic_string_view<T1, T2> &a0,
        typename std::basic_string_view<T1, T2>::size_type a1,
        typename std::basic_string_view<T1, T2>::size_type a2,
        std::basic_string_view<T1, T2> a3) {
  return a0.compare(a1, a2, a3);
}

template <typename T1, typename T2>
int f34(const std::basic_string_view<T1, T2> &a0,
        typename std::basic_string_view<T1, T2>::size_type a1,
        typename std::basic_string_view<T1, T2>::size_type a2,
        std::basic_string_view<T1, T2> a3,
        typename std::basic_string_view<T1, T2>::size_type a4,
        typename std::basic_string_view<T1, T2>::size_type a5) {
  return a0.compare(a1, a2, a3, a4, a5);
}

template <typename T1, typename T2>
int f35(const std::basic_string_view<T1, T2> &a0, const T1 *a1) {
  return a0.compare(a1);
}

template <typename T1, typename T2>
int f36(const std::basic_string_view<T1, T2> &a0,
        typename std::basic_string_view<T1, T2>::size_type a1,
        typename std::basic_string_view<T1, T2>::size_type a2, const T1 *a3) {
  return a0.compare(a1, a2, a3);
}

template <typename T1, typename T2>
int f37(const std::basic_string_view<T1, T2> &a0,
        typename std::basic_string_view<T1, T2>::size_type a1,
        typename std::basic_string_view<T1, T2>::size_type a2, const T1 *a3,
        typename std::basic_string_view<T1, T2>::size_type a4) {
  return a0.compare(a1, a2, a3, a4);
}

template <typename T1, typename T2>
typename std::basic_string_view<T1, T2>::size_type
f38(const std::basic_string_view<T1, T2> &a0, std::basic_string_view<T1, T2> a1,
    typename std::basic_string_view<T1, T2>::size_type a2) {
  return a0.find(a1, a2);
}

template <typename T1, typename T2>
typename std::basic_string_view<T1, T2>::size_type
f39(const std::basic_string_view<T1, T2> &a0, T1 a1,
    typename std::basic_string_view<T1, T2>::size_type a2) {
  return a0.find(a1, a2);
}

template <typename T1, typename T2>
typename std::basic_string_view<T1, T2>::size_type
f40(const std::basic_string_view<T1, T2> &a0, const T1 *a1,
    typename std::basic_string_view<T1, T2>::size_type a2,
    typename std::basic_string_view<T1, T2>::size_type a3) {
  return a0.find(a1, a2, a3);
}

template <typename T1, typename T2>
typename std::basic_string_view<T1, T2>::size_type
f41(const std::basic_string_view<T1, T2> &a0, const T1 *a1,
    typename std::basic_string_view<T1, T2>::size_type a2) {
  return a0.find(a1, a2);
}

template <typename T1, typename T2>
typename std::basic_string_view<T1, T2>::size_type
f42(const std::basic_string_view<T1, T2> &a0, std::basic_string_view<T1, T2> a1,
    typename std::basic_string_view<T1, T2>::size_type a2) {
  return a0.rfind(a1, a2);
}

template <typename T1, typename T2>
typename std::basic_string_view<T1, T2>::size_type
f43(const std::basic_string_view<T1, T2> &a0, T1 a1,
    typename std::basic_string_view<T1, T2>::size_type a2) {
  return a0.rfind(a1, a2);
}

template <typename T1, typename T2>
typename std::basic_string_view<T1, T2>::size_type
f44(const std::basic_string_view<T1, T2> &a0, const T1 *a1,
    typename std::basic_string_view<T1, T2>::size_type a2,
    typename std::basic_string_view<T1, T2>::size_type a3) {
  return a0.rfind(a1, a2, a3);
}

template <typename T1, typename T2>
typename std::basic_string_view<T1, T2>::size_type
f45(const std::basic_string_view<T1, T2> &a0, const T1 *a1,
    typename std::basic_string_view<T1, T2>::size_type a2) {
  return a0.rfind(a1, a2);
}

template <typename T1, typename T2>
typename std::basic_string_view<T1, T2>::size_type
f46(const std::basic_string_view<T1, T2> &a0, std::basic_string_view<T1, T2> a1,
    typename std::basic_string_view<T1, T2>::size_type a2) {
  return a0.find_first_of(a1, a2);
}

template <typename T1, typename T2>
typename std::basic_string_view<T1, T2>::size_type
f47(const std::basic_string_view<T1, T2> &a0, T1 a1,
    typename std::basic_string_view<T1, T2>::size_type a2) {
  return a0.find_first_of(a1, a2);
}

template <typename T1, typename T2>
typename std::basic_string_view<T1, T2>::size_type
f48(const std::basic_string_view<T1, T2> &a0, const T1 *a1,
    typename std::basic_string_view<T1, T2>::size_type a2,
    typename std::basic_string_view<T1, T2>::size_type a3) {
  return a0.find_first_of(a1, a2, a3);
}

template <typename T1, typename T2>
typename std::basic_string_view<T1, T2>::size_type
f49(const std::basic_string_view<T1, T2> &a0, const T1 *a1,
    typename std::basic_string_view<T1, T2>::size_type a2) {
  return a0.find_first_of(a1, a2);
}

template <typename T1, typename T2>
typename std::basic_string_view<T1, T2>::size_type
f50(const std::basic_string_view<T1, T2> &a0, std::basic_string_view<T1, T2> a1,
    typename std::basic_string_view<T1, T2>::size_type a2) {
  return a0.find_last_of(a1, a2);
}

template <typename T1, typename T2>
typename std::basic_string_view<T1, T2>::size_type
f51(const std::basic_string_view<T1, T2> &a0, T1 a1,
    typename std::basic_string_view<T1, T2>::size_type a2) {
  return a0.find_last_of(a1, a2);
}

template <typename T1, typename T2>
typename std::basic_string_view<T1, T2>::size_type
f52(const std::basic_string_view<T1, T2> &a0, const T1 *a1,
    typename std::basic_string_view<T1, T2>::size_type a2,
    typename std::basic_string_view<T1, T2>::size_type a3) {
  return a0.find_last_of(a1, a2, a3);
}

template <typename T1, typename T2>
typename std::basic_string_view<T1, T2>::size_type
f53(const std::basic_string_view<T1, T2> &a0, const T1 *a1,
    typename std::basic_string_view<T1, T2>::size_type a2) {
  return a0.find_last_of(a1, a2);
}

template <typename T1, typename T2>
typename std::basic_string_view<T1, T2>::size_type
f54(const std::basic_string_view<T1, T2> &a0, std::basic_string_view<T1, T2> a1,
    typename std::basic_string_view<T1, T2>::size_type a2) {
  return a0.find_first_not_of(a1, a2);
}

template <typename T1, typename T2>
typename std::basic_string_view<T1, T2>::size_type
f55(const std::basic_string_view<T1, T2> &a0, T1 a1,
    typename std::basic_string_view<T1, T2>::size_type a2) {
  return a0.find_first_not_of(a1, a2);
}

template <typename T1, typename T2>
typename std::basic_string_view<T1, T2>::size_type
f56(const std::basic_string_view<T1, T2> &a0, const T1 *a1,
    typename std::basic_string_view<T1, T2>::size_type a2,
    typename std::basic_string_view<T1, T2>::size_type a3) {
  return a0.find_first_not_of(a1, a2, a3);
}

template <typename T1, typename T2>
typename std::basic_string_view<T1, T2>::size_type
f57(const std::basic_string_view<T1, T2> &a0, const T1 *a1,
    typename std::basic_string_view<T1, T2>::size_type a2) {
  return a0.find_first_not_of(a1, a2);
}

template <typename T1, typename T2>
typename std::basic_string_view<T1, T2>::size_type
f58(const std::basic_string_view<T1, T2> &a0, std::basic_string_view<T1, T2> a1,
    typename std::basic_string_view<T1, T2>::size_type a2) {
  return a0.find_last_not_of(a1, a2);
}

template <typename T1, typename T2>
typename std::basic_string_view<T1, T2>::size_type
f59(const std::basic_string_view<T1, T2> &a0, T1 a1,
    typename std::basic_string_view<T1, T2>::size_type a2) {
  return a0.find_last_not_of(a1, a2);
}

template <typename T1, typename T2>
typename std::basic_string_view<T1, T2>::size_type
f60(const std::basic_string_view<T1, T2> &a0, const T1 *a1,
    typename std::basic_string_view<T1, T2>::size_type a2,
    typename std::basic_string_view<T1, T2>::size_type a3) {
  return a0.find_last_not_of(a1, a2, a3);
}

template <typename T1, typename T2>
typename std::basic_string_view<T1, T2>::size_type
f61(const std::basic_string_view<T1, T2> &a0, const T1 *a1,
    typename std::basic_string_view<T1, T2>::size_type a2) {
  return a0.find_last_not_of(a1, a2);
}

template <typename T1, typename T2>
using t16 =
    typename std::basic_string_view<T1,
                                    T2>::const_reverse_iterator::iterator_type;

template <typename T1, typename T2>
using t17 =
    typename std::basic_string_view<T1, T2>::const_reverse_iterator::pointer;

template <typename T1, typename T2>
using t18 = typename std::basic_string_view<
    T1, T2>::const_reverse_iterator::difference_type;

template <typename T1, typename T2>
using t19 =
    typename std::basic_string_view<T1, T2>::const_reverse_iterator::reference;

template <typename T1, typename T2>
using t20 =
    typename std::basic_string_view<T1, T2>::reverse_iterator::iterator_type;

template <typename T1, typename T2>
using t21 = typename std::basic_string_view<T1, T2>::reverse_iterator::pointer;

template <typename T1, typename T2>
using t22 =
    typename std::basic_string_view<T1, T2>::reverse_iterator::difference_type;

template <typename T1, typename T2>
using t23 =
    typename std::basic_string_view<T1, T2>::reverse_iterator::reference;

template <typename T1, typename T2>
typename std::basic_string_view<T1, T2>::size_type f62() {
  return std::basic_string_view<T1, T2>::npos;
}
