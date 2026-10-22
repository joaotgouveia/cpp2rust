#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <ios>
#include <string>
#include <utility>

template <typename T1> using t1 = std::numpunct<T1>;

template <typename T1> using t2 = typename std::numpunct<T1>::char_type;

template <typename T1> using t3 = typename std::numpunct<T1>::string_type;

template <typename T1> std::numpunct<T1> f1(std::size_t a0) {
  return std::numpunct<T1>(a0);
}

template <typename T1>
typename std::numpunct<T1>::char_type f2(const std::numpunct<T1> &a0) {
  return a0.decimal_point();
}

template <typename T1>
typename std::numpunct<T1>::char_type f3(const std::numpunct<T1> &a0) {
  return a0.thousands_sep();
}

template <typename T1> std::string f4(const std::numpunct<T1> &a0) {
  return a0.grouping();
}

template <typename T1>
typename std::numpunct<T1>::string_type f5(const std::numpunct<T1> &a0) {
  return a0.truename();
}

template <typename T1>
typename std::numpunct<T1>::string_type f6(const std::numpunct<T1> &a0) {
  return a0.falsename();
}

template <typename T1>
using t4 = typename std::numpunct<T1>::string_type::traits_type;

template <typename T1>
using t5 = typename std::numpunct<T1>::string_type::value_type;

template <typename T1>
using t6 = typename std::numpunct<T1>::string_type::allocator_type;

template <typename T1>
using t7 = typename std::numpunct<T1>::string_type::size_type;

template <typename T1>
using t8 = typename std::numpunct<T1>::string_type::difference_type;

template <typename T1>
using t9 = typename std::numpunct<T1>::string_type::reference;

template <typename T1>
using t10 = typename std::numpunct<T1>::string_type::const_reference;

template <typename T1>
using t11 = typename std::numpunct<T1>::string_type::pointer;

template <typename T1>
using t12 = typename std::numpunct<T1>::string_type::const_pointer;

template <typename T1>
typename std::numpunct<T1>::string_type f7(const std::allocator<T1> &a0) {
  return typename std::numpunct<T1>::string_type(a0);
}

template <typename T1>
typename std::numpunct<T1>::string_type
f8(const typename std::numpunct<T1>::string_type &a0,
   typename std::numpunct<T1>::string_type::size_type a1,
   const std::allocator<T1> &a2) {
  return typename std::numpunct<T1>::string_type(a0, a1, a2);
}

template <typename T1>
typename std::numpunct<T1>::string_type
f9(const typename std::numpunct<T1>::string_type &a0,
   typename std::numpunct<T1>::string_type::size_type a1,
   typename std::numpunct<T1>::string_type::size_type a2,
   const std::allocator<T1> &a3) {
  return typename std::numpunct<T1>::string_type(a0, a1, a2, a3);
}

template <typename T1>
typename std::numpunct<T1>::string_type
f10(T1 const *a0, typename std::numpunct<T1>::string_type::size_type a1,
    const std::allocator<T1> &a2) {
  return typename std::numpunct<T1>::string_type(a0, a1, a2);
}

template <typename T1>
typename std::numpunct<T1>::string_type f11(std::initializer_list<T1> a0,
                                            const std::allocator<T1> &a1) {
  return typename std::numpunct<T1>::string_type(a0, a1);
}

template <typename T1>
typename std::numpunct<T1>::string_type
f12(const typename std::numpunct<T1>::string_type &a0,
    const std::allocator<T1> &a1) {
  return typename std::numpunct<T1>::string_type(a0, a1);
}

template <typename T1>
typename std::numpunct<T1>::string_type
f13(typename std::numpunct<T1>::string_type &&a0,
    const std::allocator<T1> &a1) {
  return typename std::numpunct<T1>::string_type(std::move(a0), a1);
}

template <typename T1>
typename std::numpunct<T1>::string_type &
f14(typename std::numpunct<T1>::string_type &a0, T1 const *a1) {
  return a0.operator=(a1);
}

template <typename T1>
typename std::numpunct<T1>::string_type &
f15(typename std::numpunct<T1>::string_type &a0, T1 a1) {
  return a0.operator=(a1);
}

template <typename T1>
typename std::numpunct<T1>::string_type &
f16(typename std::numpunct<T1>::string_type &a0, std::initializer_list<T1> a1) {
  return a0.operator=(a1);
}

template <typename T1>
void f17(typename std::numpunct<T1>::string_type &a0,
         typename std::numpunct<T1>::string_type::size_type a1, T1 a2) {
  return a0.resize(a1, a2);
}

template <typename T1>
typename std::numpunct<T1>::string_type &
f18(typename std::numpunct<T1>::string_type &a0, T1 const *a1) {
  return a0.operator+=(a1);
}

template <typename T1>
typename std::numpunct<T1>::string_type &
f19(typename std::numpunct<T1>::string_type &a0, T1 a1) {
  return a0.operator+=(a1);
}

template <typename T1>
typename std::numpunct<T1>::string_type &
f20(typename std::numpunct<T1>::string_type &a0, std::initializer_list<T1> a1) {
  return a0.operator+=(a1);
}

template <typename T1>
typename std::numpunct<T1>::string_type &
f21(typename std::numpunct<T1>::string_type &a0, T1 const *a1,
    typename std::numpunct<T1>::string_type::size_type a2) {
  return a0.append(a1, a2);
}

template <typename T1>
typename std::numpunct<T1>::string_type &
f22(typename std::numpunct<T1>::string_type &a0, T1 const *a1) {
  return a0.append(a1);
}

template <typename T1>
typename std::numpunct<T1>::string_type &
f23(typename std::numpunct<T1>::string_type &a0,
    typename std::numpunct<T1>::string_type::size_type a1, T1 a2) {
  return a0.append(a1, a2);
}

template <typename T1>
typename std::numpunct<T1>::string_type &
f24(typename std::numpunct<T1>::string_type &a0, std::initializer_list<T1> a1) {
  return a0.append(a1);
}

template <typename T1>
void f25(typename std::numpunct<T1>::string_type &a0, T1 a1) {
  return a0.push_back(a1);
}

template <typename T1>
typename std::numpunct<T1>::string_type &
f26(typename std::numpunct<T1>::string_type &a0, T1 const *a1,
    typename std::numpunct<T1>::string_type::size_type a2) {
  return a0.assign(a1, a2);
}

template <typename T1>
typename std::numpunct<T1>::string_type &
f27(typename std::numpunct<T1>::string_type &a0, T1 const *a1) {
  return a0.assign(a1);
}

template <typename T1>
typename std::numpunct<T1>::string_type &
f28(typename std::numpunct<T1>::string_type &a0,
    typename std::numpunct<T1>::string_type::size_type a1, T1 a2) {
  return a0.assign(a1, a2);
}

template <typename T1>
typename std::numpunct<T1>::string_type &
f29(typename std::numpunct<T1>::string_type &a0, std::initializer_list<T1> a1) {
  return a0.assign(a1);
}

template <typename T1>
typename std::numpunct<T1>::string_type::iterator
f30(typename std::numpunct<T1>::string_type &a0,
    typename std::numpunct<T1>::string_type::const_iterator a1,
    typename std::numpunct<T1>::string_type::size_type a2, T1 a3) {
  return a0.insert(a1, a2, a3);
}

template <typename T1>
typename std::numpunct<T1>::string_type::iterator
f31(typename std::numpunct<T1>::string_type &a0,
    typename std::numpunct<T1>::string_type::const_iterator a1,
    std::initializer_list<T1> a2) {
  return a0.insert(a1, a2);
}

template <typename T1>
typename std::numpunct<T1>::string_type &
f32(typename std::numpunct<T1>::string_type &a0,
    typename std::numpunct<T1>::string_type::size_type a1, T1 const *a2,
    typename std::numpunct<T1>::string_type::size_type a3) {
  return a0.insert(a1, a2, a3);
}

template <typename T1>
typename std::numpunct<T1>::string_type &
f33(typename std::numpunct<T1>::string_type &a0,
    typename std::numpunct<T1>::string_type::size_type a1, T1 const *a2) {
  return a0.insert(a1, a2);
}

template <typename T1>
typename std::numpunct<T1>::string_type &
f34(typename std::numpunct<T1>::string_type &a0,
    typename std::numpunct<T1>::string_type::size_type a1,
    typename std::numpunct<T1>::string_type::size_type a2, T1 a3) {
  return a0.insert(a1, a2, a3);
}

template <typename T1>
typename std::numpunct<T1>::string_type::iterator
f35(typename std::numpunct<T1>::string_type &a0,
    typename std::numpunct<T1>::string_type::const_iterator a1, T1 a2) {
  return a0.insert(a1, a2);
}

template <typename T1>
typename std::numpunct<T1>::string_type &
f36(typename std::numpunct<T1>::string_type &a0,
    typename std::numpunct<T1>::string_type::size_type a1,
    typename std::numpunct<T1>::string_type::size_type a2, T1 const *a3,
    typename std::numpunct<T1>::string_type::size_type a4) {
  return a0.replace(a1, a2, a3, a4);
}

template <typename T1>
typename std::numpunct<T1>::string_type &
f37(typename std::numpunct<T1>::string_type &a0,
    typename std::numpunct<T1>::string_type::size_type a1,
    typename std::numpunct<T1>::string_type::size_type a2, T1 const *a3) {
  return a0.replace(a1, a2, a3);
}

template <typename T1>
typename std::numpunct<T1>::string_type &
f38(typename std::numpunct<T1>::string_type &a0,
    typename std::numpunct<T1>::string_type::size_type a1,
    typename std::numpunct<T1>::string_type::size_type a2,
    typename std::numpunct<T1>::string_type::size_type a3, T1 a4) {
  return a0.replace(a1, a2, a3, a4);
}

template <typename T1>
typename std::numpunct<T1>::string_type &
f39(typename std::numpunct<T1>::string_type &a0,
    typename std::numpunct<T1>::string_type::const_iterator a1,
    typename std::numpunct<T1>::string_type::const_iterator a2, T1 const *a3,
    typename std::numpunct<T1>::string_type::size_type a4) {
  return a0.replace(a1, a2, a3, a4);
}

template <typename T1>
typename std::numpunct<T1>::string_type &
f40(typename std::numpunct<T1>::string_type &a0,
    typename std::numpunct<T1>::string_type::const_iterator a1,
    typename std::numpunct<T1>::string_type::const_iterator a2, T1 const *a3) {
  return a0.replace(a1, a2, a3);
}

template <typename T1>
typename std::numpunct<T1>::string_type &
f41(typename std::numpunct<T1>::string_type &a0,
    typename std::numpunct<T1>::string_type::const_iterator a1,
    typename std::numpunct<T1>::string_type::const_iterator a2,
    typename std::numpunct<T1>::string_type::size_type a3, T1 a4) {
  return a0.replace(a1, a2, a3, a4);
}

template <typename T1>
typename std::numpunct<T1>::string_type &
f42(typename std::numpunct<T1>::string_type &a0,
    typename std::numpunct<T1>::string_type::const_iterator a1,
    typename std::numpunct<T1>::string_type::const_iterator a2, T1 *a3,
    T1 *a4) {
  return a0.replace(a1, a2, a3, a4);
}

template <typename T1>
typename std::numpunct<T1>::string_type &
f43(typename std::numpunct<T1>::string_type &a0,
    typename std::numpunct<T1>::string_type::const_iterator a1,
    typename std::numpunct<T1>::string_type::const_iterator a2, T1 const *a3,
    T1 const *a4) {
  return a0.replace(a1, a2, a3, a4);
}

template <typename T1>
typename std::numpunct<T1>::string_type &
f44(typename std::numpunct<T1>::string_type &a0,
    typename std::numpunct<T1>::string_type::const_iterator a1,
    typename std::numpunct<T1>::string_type::const_iterator a2,
    std::initializer_list<T1> a3) {
  return a0.replace(a1, a2, a3);
}

template <typename T1>
typename std::numpunct<T1>::string_type::size_type
f45(const typename std::numpunct<T1>::string_type &a0, T1 *a1,
    typename std::numpunct<T1>::string_type::size_type a2,
    typename std::numpunct<T1>::string_type::size_type a3) {
  return a0.copy(a1, a2, a3);
}

template <typename T1>
T1 const *f46(const typename std::numpunct<T1>::string_type &a0) {
  return a0.c_str();
}

template <typename T1>
T1 const *f47(const typename std::numpunct<T1>::string_type &a0) {
  return a0.data();
}

template <typename T1>
typename std::numpunct<T1>::string_type::size_type
f48(const typename std::numpunct<T1>::string_type &a0, T1 const *a1,
    typename std::numpunct<T1>::string_type::size_type a2,
    typename std::numpunct<T1>::string_type::size_type a3) {
  return a0.find(a1, a2, a3);
}

template <typename T1>
typename std::numpunct<T1>::string_type::size_type
f49(const typename std::numpunct<T1>::string_type &a0, T1 const *a1,
    typename std::numpunct<T1>::string_type::size_type a2) {
  return a0.find(a1, a2);
}

template <typename T1>
typename std::numpunct<T1>::string_type::size_type
f50(const typename std::numpunct<T1>::string_type &a0, T1 a1,
    typename std::numpunct<T1>::string_type::size_type a2) {
  return a0.find(a1, a2);
}

template <typename T1>
typename std::numpunct<T1>::string_type::size_type
f51(const typename std::numpunct<T1>::string_type &a0, T1 const *a1,
    typename std::numpunct<T1>::string_type::size_type a2,
    typename std::numpunct<T1>::string_type::size_type a3) {
  return a0.rfind(a1, a2, a3);
}

template <typename T1>
typename std::numpunct<T1>::string_type::size_type
f52(const typename std::numpunct<T1>::string_type &a0, T1 const *a1,
    typename std::numpunct<T1>::string_type::size_type a2) {
  return a0.rfind(a1, a2);
}

template <typename T1>
typename std::numpunct<T1>::string_type::size_type
f53(const typename std::numpunct<T1>::string_type &a0, T1 a1,
    typename std::numpunct<T1>::string_type::size_type a2) {
  return a0.rfind(a1, a2);
}

template <typename T1>
typename std::numpunct<T1>::string_type::size_type
f54(const typename std::numpunct<T1>::string_type &a0, T1 const *a1,
    typename std::numpunct<T1>::string_type::size_type a2,
    typename std::numpunct<T1>::string_type::size_type a3) {
  return a0.find_first_of(a1, a2, a3);
}

template <typename T1>
typename std::numpunct<T1>::string_type::size_type
f55(const typename std::numpunct<T1>::string_type &a0, T1 const *a1,
    typename std::numpunct<T1>::string_type::size_type a2) {
  return a0.find_first_of(a1, a2);
}

template <typename T1>
typename std::numpunct<T1>::string_type::size_type
f56(const typename std::numpunct<T1>::string_type &a0, T1 a1,
    typename std::numpunct<T1>::string_type::size_type a2) {
  return a0.find_first_of(a1, a2);
}

template <typename T1>
typename std::numpunct<T1>::string_type::size_type
f57(const typename std::numpunct<T1>::string_type &a0, T1 const *a1,
    typename std::numpunct<T1>::string_type::size_type a2,
    typename std::numpunct<T1>::string_type::size_type a3) {
  return a0.find_last_of(a1, a2, a3);
}

template <typename T1>
typename std::numpunct<T1>::string_type::size_type
f58(const typename std::numpunct<T1>::string_type &a0, T1 const *a1,
    typename std::numpunct<T1>::string_type::size_type a2) {
  return a0.find_last_of(a1, a2);
}

template <typename T1>
typename std::numpunct<T1>::string_type::size_type
f59(const typename std::numpunct<T1>::string_type &a0, T1 a1,
    typename std::numpunct<T1>::string_type::size_type a2) {
  return a0.find_last_of(a1, a2);
}

template <typename T1>
typename std::numpunct<T1>::string_type::size_type
f60(const typename std::numpunct<T1>::string_type &a0, T1 const *a1,
    typename std::numpunct<T1>::string_type::size_type a2,
    typename std::numpunct<T1>::string_type::size_type a3) {
  return a0.find_first_not_of(a1, a2, a3);
}

template <typename T1>
typename std::numpunct<T1>::string_type::size_type
f61(const typename std::numpunct<T1>::string_type &a0, T1 const *a1,
    typename std::numpunct<T1>::string_type::size_type a2) {
  return a0.find_first_not_of(a1, a2);
}

template <typename T1>
typename std::numpunct<T1>::string_type::size_type
f62(const typename std::numpunct<T1>::string_type &a0, T1 a1,
    typename std::numpunct<T1>::string_type::size_type a2) {
  return a0.find_first_not_of(a1, a2);
}

template <typename T1>
typename std::numpunct<T1>::string_type::size_type
f63(const typename std::numpunct<T1>::string_type &a0, T1 const *a1,
    typename std::numpunct<T1>::string_type::size_type a2,
    typename std::numpunct<T1>::string_type::size_type a3) {
  return a0.find_last_not_of(a1, a2, a3);
}

template <typename T1>
typename std::numpunct<T1>::string_type::size_type
f64(const typename std::numpunct<T1>::string_type &a0, T1 const *a1,
    typename std::numpunct<T1>::string_type::size_type a2) {
  return a0.find_last_not_of(a1, a2);
}

template <typename T1>
typename std::numpunct<T1>::string_type::size_type
f65(const typename std::numpunct<T1>::string_type &a0, T1 a1,
    typename std::numpunct<T1>::string_type::size_type a2) {
  return a0.find_last_not_of(a1, a2);
}

template <typename T1>
int f66(const typename std::numpunct<T1>::string_type &a0, T1 const *a1) {
  return a0.compare(a1);
}

template <typename T1>
int f67(const typename std::numpunct<T1>::string_type &a0,
        typename std::numpunct<T1>::string_type::size_type a1,
        typename std::numpunct<T1>::string_type::size_type a2, T1 const *a3) {
  return a0.compare(a1, a2, a3);
}

template <typename T1>
int f68(const typename std::numpunct<T1>::string_type &a0,
        typename std::numpunct<T1>::string_type::size_type a1,
        typename std::numpunct<T1>::string_type::size_type a2, T1 const *a3,
        typename std::numpunct<T1>::string_type::size_type a4) {
  return a0.compare(a1, a2, a3, a4);
}

template <typename T1, typename T2,
          typename T3 = typename std::enable_if<
              std::is_convertible<
                  typename std::iterator_traits<T2>::iterator_category,
                  std::input_iterator_tag>::value,
              void>::type>
typename std::numpunct<T1>::string_type f69(T2 a0, T2 a1,
                                            const std::allocator<T1> &a2) {
  return typename std::numpunct<T1>::string_type(a0, a1, a2);
}

template <typename T1> const std::locale::id &f70() {
  return std::numpunct<T1>::id;
}
