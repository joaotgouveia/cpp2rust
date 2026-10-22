#include "../common/init.h"
#include "../common/traits.h"
#include <cstddef>
#include <initializer_list>
#include <ios>
#include <locale>
#include <memory>
#include <string.h>
#include <string>
#include <string_view>
#include <utility>

template <typename T1>
typename std::messages_byname<T1>::string_type
f88(typename std::messages_byname<T1>::string_type &&a0,
    typename std::messages_byname<T1>::string_type::size_type a1,
    const std::allocator<T1> &a2) {
  return typename std::messages_byname<T1>::string_type(std::move(a0), a1, a2);
}

template <typename T1>
typename std::messages_byname<T1>::string_type
f89(typename std::messages_byname<T1>::string_type &&a0,
    typename std::messages_byname<T1>::string_type::size_type a1,
    typename std::messages_byname<T1>::string_type::size_type a2,
    const std::allocator<T1> &a3) {
  return
      typename std::messages_byname<T1>::string_type(std::move(a0), a1, a2, a3);
}
