#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <ios>
#include <locale>
#include <string>
#include <utility>

template <typename T1>
std::basic_string_view<T1, std::char_traits<T1>>
f75(const typename std::messages_byname<T1>::string_type &a0) {
  return static_cast<std::basic_string_view<T1, std::char_traits<T1>>>(a0);
}

template <typename T1>
T1 *f76(typename std::messages_byname<T1>::string_type &a0) {
  return a0.data();
}

template <typename T1, typename T2>
typename std::messages_byname<T1>::string_type
f77(const T2 &a0, typename std::messages_byname<T1>::string_type::size_type a1,
    typename std::messages_byname<T1>::string_type::size_type a2,
    const std::allocator<T1> &a3) {
  return typename std::messages_byname<T1>::string_type(a0, a1, a2, a3);
}

template <typename T1, typename T2>
typename std::messages_byname<T1>::string_type
f78(const T2 &a0, const std::allocator<T1> &a1) {
  return typename std::messages_byname<T1>::string_type(a0, a1);
}
