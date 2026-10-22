#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <ios>
#include <locale>
#include <string>
#include <utility>

template <typename T1>
typename std::messages_byname<T1>::string_type
f73(T1 const *a0, const std::allocator<T1> &a1) {
  return typename std::messages_byname<T1>::string_type(a0, a1);
}

template <typename T1>
typename std::messages_byname<T1>::string_type
f74(typename std::messages_byname<T1>::string_type::size_type a0, T1 a1,
    const std::allocator<T1> &a2) {
  return typename std::messages_byname<T1>::string_type(a0, a1, a2);
}
