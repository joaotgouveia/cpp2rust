#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <cstddef>
#include <initializer_list>
#include <ios>
#include <locale>
#include <memory>
#include <string>
#include <string_view>
#include <utility>

template <typename T1> using t1 = std::messages_byname<T1>;

template <typename T1>
using t2 = typename std::messages_byname<T1>::string_type;

template <typename T1>
std::messages_byname<T1> f1(const char *a0, std::size_t a1) {
  return std::messages_byname<T1>(a0, a1);
}

template <typename T1>
std::messages_byname<T1> f2(const std::string &a0, std::size_t a1) {
  return std::messages_byname<T1>(a0, a1);
}

template <typename T1>
typename std::messages_byname<T1>::catalog
f3(const std::messages_byname<T1> &a0, const std::basic_string<char> &a1,
   const std::locale &a2) {
  return a0.open(a1, a2);
}

template <typename T1>
typename std::messages_byname<T1>::string_type
f4(const std::messages_byname<T1> &a0,
   typename std::messages_byname<T1>::catalog a1, int a2, int a3,
   const typename std::messages_byname<T1>::string_type &a4) {
  return a0.get(a1, a2, a3, a4);
}

template <typename T1>
void f5(const std::messages_byname<T1> &a0,
        typename std::messages_byname<T1>::catalog a1) {
  return a0.close(a1);
}

template <typename T1>
using t3 = typename std::messages_byname<T1>::string_type::traits_type;

template <typename T1>
using t4 = typename std::messages_byname<T1>::string_type::value_type;

template <typename T1>
using t5 = typename std::messages_byname<T1>::string_type::allocator_type;

template <typename T1>
using t6 = typename std::messages_byname<T1>::string_type::size_type;

template <typename T1>
using t7 = typename std::messages_byname<T1>::string_type::difference_type;

template <typename T1>
using t8 = typename std::messages_byname<T1>::string_type::reference;

template <typename T1>
using t9 = typename std::messages_byname<T1>::string_type::const_reference;

template <typename T1>
using t10 = typename std::messages_byname<T1>::string_type::pointer;

template <typename T1>
using t11 = typename std::messages_byname<T1>::string_type::const_pointer;

template <typename T1>
typename std::messages_byname<T1>::string_type
f6(const typename std::messages_byname<T1>::string_type &a0,
   typename std::messages_byname<T1>::string_type::size_type a1,
   const std::allocator<T1> &a2) {
  return typename std::messages_byname<T1>::string_type(a0, a1, a2);
}

template <typename T1>
typename std::messages_byname<T1>::string_type
f7(const typename std::messages_byname<T1>::string_type &a0,
   typename std::messages_byname<T1>::string_type::size_type a1,
   typename std::messages_byname<T1>::string_type::size_type a2,
   const std::allocator<T1> &a3) {
  return typename std::messages_byname<T1>::string_type(a0, a1, a2, a3);
}

template <typename T1>
typename std::messages_byname<T1>::string_type
f8(T1 const *a0, typename std::messages_byname<T1>::string_type::size_type a1,
   const std::allocator<T1> &a2) {
  return typename std::messages_byname<T1>::string_type(a0, a1, a2);
}

template <typename T1>
typename std::messages_byname<T1>::string_type
f9(T1 const *a0, const std::allocator<T1> &a1) {
  return typename std::messages_byname<T1>::string_type(a0, a1);
}

template <typename T1>
typename std::messages_byname<T1>::string_type
f10(typename std::messages_byname<T1>::string_type::size_type a0, T1 a1,
    const std::allocator<T1> &a2) {
  return typename std::messages_byname<T1>::string_type(a0, a1, a2);
}

template <typename T1>
typename std::messages_byname<T1>::string_type
f11(std::initializer_list<T1> a0, const std::allocator<T1> &a1) {
  return typename std::messages_byname<T1>::string_type(a0, a1);
}
