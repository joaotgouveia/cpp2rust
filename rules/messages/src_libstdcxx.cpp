#include <initializer_list>
#include <ios>
#include <locale>
#include <string>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1>
typename std::messages<T1>::catalog f12(const std::messages<T1> &a0,
                                        const std::basic_string<char> &a1,
                                        const std::locale &a2, const char *a3) {
  return a0.open(a1, a2, a3);
}

template <typename T1>
typename std::messages<T1>::string_type f13(const std::allocator<T1> &a0) {
  return typename std::messages<T1>::string_type(a0);
}

template <typename T1>
typename std::messages<T1>::string_type
f14(const typename std::messages<T1>::string_type &a0,
    const std::allocator<T1> &a1) {
  return typename std::messages<T1>::string_type(a0, a1);
}

template <typename T1>
typename std::messages<T1>::string_type
f15(typename std::messages<T1>::string_type &&a0,
    const std::allocator<T1> &a1) {
  return typename std::messages<T1>::string_type(std::move(a0), a1);
}

template <typename T1>
typename std::messages<T1>::string_type &
f16(typename std::messages<T1>::string_type &a0, T1 const *a1) {
  return a0.operator=(a1);
}

template <typename T1>
typename std::messages<T1>::string_type &
f17(typename std::messages<T1>::string_type &a0, T1 a1) {
  return a0.operator=(a1);
}

template <typename T1>
typename std::messages<T1>::string_type &
f18(typename std::messages<T1>::string_type &a0, std::initializer_list<T1> a1) {
  return a0.operator=(a1);
}

template <typename T1>
void f19(typename std::messages<T1>::string_type &a0,
         typename std::messages<T1>::string_type::size_type a1, T1 a2) {
  return a0.resize(a1, a2);
}

template <typename T1>
typename std::messages<T1>::string_type &
f20(typename std::messages<T1>::string_type &a0, T1 const *a1) {
  return a0.operator+=(a1);
}

template <typename T1>
typename std::messages<T1>::string_type &
f21(typename std::messages<T1>::string_type &a0, T1 a1) {
  return a0.operator+=(a1);
}

template <typename T1>
typename std::messages<T1>::string_type &
f22(typename std::messages<T1>::string_type &a0, std::initializer_list<T1> a1) {
  return a0.operator+=(a1);
}

template <typename T1>
typename std::messages<T1>::string_type &
f23(typename std::messages<T1>::string_type &a0, T1 const *a1,
    typename std::messages<T1>::string_type::size_type a2) {
  return a0.append(a1, a2);
}

template <typename T1>
typename std::messages<T1>::string_type &
f24(typename std::messages<T1>::string_type &a0, T1 const *a1) {
  return a0.append(a1);
}

template <typename T1>
typename std::messages<T1>::string_type &
f25(typename std::messages<T1>::string_type &a0,
    typename std::messages<T1>::string_type::size_type a1, T1 a2) {
  return a0.append(a1, a2);
}

template <typename T1>
typename std::messages<T1>::string_type &
f26(typename std::messages<T1>::string_type &a0, std::initializer_list<T1> a1) {
  return a0.append(a1);
}

template <typename T1>
void f27(typename std::messages<T1>::string_type &a0, T1 a1) {
  return a0.push_back(a1);
}

template <typename T1>
typename std::messages<T1>::string_type &
f28(typename std::messages<T1>::string_type &a0, T1 const *a1,
    typename std::messages<T1>::string_type::size_type a2) {
  return a0.assign(a1, a2);
}

template <typename T1>
typename std::messages<T1>::string_type &
f29(typename std::messages<T1>::string_type &a0, T1 const *a1) {
  return a0.assign(a1);
}

template <typename T1>
typename std::messages<T1>::string_type &
f30(typename std::messages<T1>::string_type &a0,
    typename std::messages<T1>::string_type::size_type a1, T1 a2) {
  return a0.assign(a1, a2);
}

template <typename T1>
typename std::messages<T1>::string_type &
f31(typename std::messages<T1>::string_type &a0, std::initializer_list<T1> a1) {
  return a0.assign(a1);
}

template <typename T1>
typename std::messages<T1>::string_type::iterator
f32(typename std::messages<T1>::string_type &a0,
    typename std::messages<T1>::string_type::const_iterator a1,
    typename std::messages<T1>::string_type::size_type a2, T1 a3) {
  return a0.insert(a1, a2, a3);
}

template <typename T1>
typename std::messages<T1>::string_type::iterator
f33(typename std::messages<T1>::string_type &a0,
    typename std::messages<T1>::string_type::const_iterator a1,
    std::initializer_list<T1> a2) {
  return a0.insert(a1, a2);
}

template <typename T1>
typename std::messages<T1>::string_type &
f34(typename std::messages<T1>::string_type &a0,
    typename std::messages<T1>::string_type::size_type a1, T1 const *a2,
    typename std::messages<T1>::string_type::size_type a3) {
  return a0.insert(a1, a2, a3);
}

template <typename T1>
typename std::messages<T1>::string_type &
f35(typename std::messages<T1>::string_type &a0,
    typename std::messages<T1>::string_type::size_type a1, T1 const *a2) {
  return a0.insert(a1, a2);
}

template <typename T1>
typename std::messages<T1>::string_type &
f36(typename std::messages<T1>::string_type &a0,
    typename std::messages<T1>::string_type::size_type a1,
    typename std::messages<T1>::string_type::size_type a2, T1 a3) {
  return a0.insert(a1, a2, a3);
}

template <typename T1>
typename std::messages<T1>::string_type::iterator
f37(typename std::messages<T1>::string_type &a0,
    typename std::messages<T1>::string_type::const_iterator a1, T1 a2) {
  return a0.insert(a1, a2);
}

template <typename T1>
typename std::messages<T1>::string_type &
f38(typename std::messages<T1>::string_type &a0,
    typename std::messages<T1>::string_type::size_type a1,
    typename std::messages<T1>::string_type::size_type a2, T1 const *a3,
    typename std::messages<T1>::string_type::size_type a4) {
  return a0.replace(a1, a2, a3, a4);
}

template <typename T1>
typename std::messages<T1>::string_type &
f39(typename std::messages<T1>::string_type &a0,
    typename std::messages<T1>::string_type::size_type a1,
    typename std::messages<T1>::string_type::size_type a2, T1 const *a3) {
  return a0.replace(a1, a2, a3);
}

template <typename T1>
typename std::messages<T1>::string_type &
f40(typename std::messages<T1>::string_type &a0,
    typename std::messages<T1>::string_type::size_type a1,
    typename std::messages<T1>::string_type::size_type a2,
    typename std::messages<T1>::string_type::size_type a3, T1 a4) {
  return a0.replace(a1, a2, a3, a4);
}

template <typename T1>
typename std::messages<T1>::string_type &
f41(typename std::messages<T1>::string_type &a0,
    typename std::messages<T1>::string_type::const_iterator a1,
    typename std::messages<T1>::string_type::const_iterator a2, T1 const *a3,
    typename std::messages<T1>::string_type::size_type a4) {
  return a0.replace(a1, a2, a3, a4);
}

template <typename T1>
typename std::messages<T1>::string_type &
f42(typename std::messages<T1>::string_type &a0,
    typename std::messages<T1>::string_type::const_iterator a1,
    typename std::messages<T1>::string_type::const_iterator a2, T1 const *a3) {
  return a0.replace(a1, a2, a3);
}

template <typename T1>
typename std::messages<T1>::string_type &
f43(typename std::messages<T1>::string_type &a0,
    typename std::messages<T1>::string_type::const_iterator a1,
    typename std::messages<T1>::string_type::const_iterator a2,
    typename std::messages<T1>::string_type::size_type a3, T1 a4) {
  return a0.replace(a1, a2, a3, a4);
}

template <typename T1>
typename std::messages<T1>::string_type &
f44(typename std::messages<T1>::string_type &a0,
    typename std::messages<T1>::string_type::const_iterator a1,
    typename std::messages<T1>::string_type::const_iterator a2, T1 *a3,
    T1 *a4) {
  return a0.replace(a1, a2, a3, a4);
}

template <typename T1>
typename std::messages<T1>::string_type &
f45(typename std::messages<T1>::string_type &a0,
    typename std::messages<T1>::string_type::const_iterator a1,
    typename std::messages<T1>::string_type::const_iterator a2, T1 const *a3,
    T1 const *a4) {
  return a0.replace(a1, a2, a3, a4);
}

template <typename T1>
typename std::messages<T1>::string_type &
f46(typename std::messages<T1>::string_type &a0,
    typename std::messages<T1>::string_type::const_iterator a1,
    typename std::messages<T1>::string_type::const_iterator a2,
    std::initializer_list<T1> a3) {
  return a0.replace(a1, a2, a3);
}

template <typename T1>
typename std::messages<T1>::string_type::size_type
f47(const typename std::messages<T1>::string_type &a0, T1 *a1,
    typename std::messages<T1>::string_type::size_type a2,
    typename std::messages<T1>::string_type::size_type a3) {
  return a0.copy(a1, a2, a3);
}

template <typename T1>
T1 const *f48(const typename std::messages<T1>::string_type &a0) {
  return a0.c_str();
}

template <typename T1>
T1 const *f49(const typename std::messages<T1>::string_type &a0) {
  return a0.data();
}

template <typename T1>
typename std::messages<T1>::string_type::size_type
f50(const typename std::messages<T1>::string_type &a0, T1 const *a1,
    typename std::messages<T1>::string_type::size_type a2,
    typename std::messages<T1>::string_type::size_type a3) {
  return a0.find(a1, a2, a3);
}

template <typename T1>
typename std::messages<T1>::string_type::size_type
f51(const typename std::messages<T1>::string_type &a0, T1 const *a1,
    typename std::messages<T1>::string_type::size_type a2) {
  return a0.find(a1, a2);
}

template <typename T1>
typename std::messages<T1>::string_type::size_type
f52(const typename std::messages<T1>::string_type &a0, T1 a1,
    typename std::messages<T1>::string_type::size_type a2) {
  return a0.find(a1, a2);
}

template <typename T1>
typename std::messages<T1>::string_type::size_type
f53(const typename std::messages<T1>::string_type &a0, T1 const *a1,
    typename std::messages<T1>::string_type::size_type a2,
    typename std::messages<T1>::string_type::size_type a3) {
  return a0.rfind(a1, a2, a3);
}

template <typename T1>
typename std::messages<T1>::string_type::size_type
f54(const typename std::messages<T1>::string_type &a0, T1 const *a1,
    typename std::messages<T1>::string_type::size_type a2) {
  return a0.rfind(a1, a2);
}

template <typename T1>
typename std::messages<T1>::string_type::size_type
f55(const typename std::messages<T1>::string_type &a0, T1 a1,
    typename std::messages<T1>::string_type::size_type a2) {
  return a0.rfind(a1, a2);
}

template <typename T1>
typename std::messages<T1>::string_type::size_type
f56(const typename std::messages<T1>::string_type &a0, T1 const *a1,
    typename std::messages<T1>::string_type::size_type a2,
    typename std::messages<T1>::string_type::size_type a3) {
  return a0.find_first_of(a1, a2, a3);
}

template <typename T1>
typename std::messages<T1>::string_type::size_type
f57(const typename std::messages<T1>::string_type &a0, T1 const *a1,
    typename std::messages<T1>::string_type::size_type a2) {
  return a0.find_first_of(a1, a2);
}

template <typename T1>
typename std::messages<T1>::string_type::size_type
f58(const typename std::messages<T1>::string_type &a0, T1 a1,
    typename std::messages<T1>::string_type::size_type a2) {
  return a0.find_first_of(a1, a2);
}

template <typename T1>
typename std::messages<T1>::string_type::size_type
f59(const typename std::messages<T1>::string_type &a0, T1 const *a1,
    typename std::messages<T1>::string_type::size_type a2,
    typename std::messages<T1>::string_type::size_type a3) {
  return a0.find_last_of(a1, a2, a3);
}

template <typename T1>
typename std::messages<T1>::string_type::size_type
f60(const typename std::messages<T1>::string_type &a0, T1 const *a1,
    typename std::messages<T1>::string_type::size_type a2) {
  return a0.find_last_of(a1, a2);
}

template <typename T1>
typename std::messages<T1>::string_type::size_type
f61(const typename std::messages<T1>::string_type &a0, T1 a1,
    typename std::messages<T1>::string_type::size_type a2) {
  return a0.find_last_of(a1, a2);
}

template <typename T1>
typename std::messages<T1>::string_type::size_type
f62(const typename std::messages<T1>::string_type &a0, T1 const *a1,
    typename std::messages<T1>::string_type::size_type a2,
    typename std::messages<T1>::string_type::size_type a3) {
  return a0.find_first_not_of(a1, a2, a3);
}

template <typename T1>
typename std::messages<T1>::string_type::size_type
f63(const typename std::messages<T1>::string_type &a0, T1 const *a1,
    typename std::messages<T1>::string_type::size_type a2) {
  return a0.find_first_not_of(a1, a2);
}

template <typename T1>
typename std::messages<T1>::string_type::size_type
f64(const typename std::messages<T1>::string_type &a0, T1 a1,
    typename std::messages<T1>::string_type::size_type a2) {
  return a0.find_first_not_of(a1, a2);
}

template <typename T1>
typename std::messages<T1>::string_type::size_type
f65(const typename std::messages<T1>::string_type &a0, T1 const *a1,
    typename std::messages<T1>::string_type::size_type a2,
    typename std::messages<T1>::string_type::size_type a3) {
  return a0.find_last_not_of(a1, a2, a3);
}

template <typename T1>
typename std::messages<T1>::string_type::size_type
f66(const typename std::messages<T1>::string_type &a0, T1 const *a1,
    typename std::messages<T1>::string_type::size_type a2) {
  return a0.find_last_not_of(a1, a2);
}

template <typename T1>
typename std::messages<T1>::string_type::size_type
f67(const typename std::messages<T1>::string_type &a0, T1 a1,
    typename std::messages<T1>::string_type::size_type a2) {
  return a0.find_last_not_of(a1, a2);
}

template <typename T1>
int f68(const typename std::messages<T1>::string_type &a0, T1 const *a1) {
  return a0.compare(a1);
}

template <typename T1>
int f69(const typename std::messages<T1>::string_type &a0,
        typename std::messages<T1>::string_type::size_type a1,
        typename std::messages<T1>::string_type::size_type a2, T1 const *a3) {
  return a0.compare(a1, a2, a3);
}

template <typename T1>
int f70(const typename std::messages<T1>::string_type &a0,
        typename std::messages<T1>::string_type::size_type a1,
        typename std::messages<T1>::string_type::size_type a2, T1 const *a3,
        typename std::messages<T1>::string_type::size_type a4) {
  return a0.compare(a1, a2, a3, a4);
}

template <typename T1, typename T2,
          typename T3 = typename std::enable_if<
              std::is_convertible<
                  typename std::iterator_traits<T2>::iterator_category,
                  std::input_iterator_tag>::value,
              void>::type>
typename std::messages<T1>::string_type f71(T2 a0, T2 a1,
                                            const std::allocator<T1> &a2) {
  return typename std::messages<T1>::string_type(a0, a1, a2);
}
