#include "../common/init.h"
#include "../common/traits.h"
#include <cstddef>
#include <initializer_list>
#include <ios>
#include <locale>
#include <memory>
#include <string>
#include <string_view>
#include <utility>

template <typename T1> using t1 = std::messages<T1>;

template <typename T1> using t2 = typename std::messages<T1>::char_type;

template <typename T1> using t3 = typename std::messages<T1>::string_type;

template <typename T1>
typename std::messages<T1>::catalog f1(const std::messages<T1> &a0,
                                       const std::basic_string<char> &a1,
                                       const std::locale &a2) {
  return a0.open(a1, a2);
}

template <typename T1>
typename std::messages<T1>::string_type
f2(const std::messages<T1> &a0, typename std::messages<T1>::catalog a1, int a2,
   int a3, const typename std::messages<T1>::string_type &a4) {
  return a0.get(a1, a2, a3, a4);
}

template <typename T1>
void f3(const std::messages<T1> &a0, typename std::messages<T1>::catalog a1) {
  return a0.close(a1);
}

template <typename T1>
using t4 = typename std::messages<T1>::string_type::traits_type;

template <typename T1>
using t5 = typename std::messages<T1>::string_type::value_type;

template <typename T1>
using t6 = typename std::messages<T1>::string_type::allocator_type;

template <typename T1>
using t7 = typename std::messages<T1>::string_type::size_type;

template <typename T1>
using t8 = typename std::messages<T1>::string_type::difference_type;

template <typename T1>
using t9 = typename std::messages<T1>::string_type::reference;

template <typename T1>
using t10 = typename std::messages<T1>::string_type::const_reference;

template <typename T1>
using t11 = typename std::messages<T1>::string_type::pointer;

template <typename T1>
using t12 = typename std::messages<T1>::string_type::const_pointer;

template <typename T1>
typename std::messages<T1>::string_type
f4(const typename std::messages<T1>::string_type &a0,
   typename std::messages<T1>::string_type::size_type a1,
   const std::allocator<T1> &a2) {
  return typename std::messages<T1>::string_type(a0, a1, a2);
}

template <typename T1>
typename std::messages<T1>::string_type
f5(const typename std::messages<T1>::string_type &a0,
   typename std::messages<T1>::string_type::size_type a1,
   typename std::messages<T1>::string_type::size_type a2,
   const std::allocator<T1> &a3) {
  return typename std::messages<T1>::string_type(a0, a1, a2, a3);
}

template <typename T1>
typename std::messages<T1>::string_type
f6(T1 const *a0, typename std::messages<T1>::string_type::size_type a1,
   const std::allocator<T1> &a2) {
  return typename std::messages<T1>::string_type(a0, a1, a2);
}

template <typename T1>
typename std::messages<T1>::string_type f7(std::initializer_list<T1> a0,
                                           const std::allocator<T1> &a1) {
  return typename std::messages<T1>::string_type(a0, a1);
}

template <typename T1> const std::locale::id &f8() {
  return std::messages<T1>::id;
}
