#include <cstddef>
#include <initializer_list>
#include <ios>
#include <locale>
#include <memory>
#include <string>
#include <string_view>
#include <utility>

template <typename T1>
typename std::messages<T1>::string_type
f93(typename std::messages<T1>::string_type &&a0,
    typename std::messages<T1>::string_type::size_type a1,
    const std::allocator<T1> &a2) {
  return typename std::messages<T1>::string_type(std::move(a0), a1, a2);
}

template <typename T1>
typename std::messages<T1>::string_type
f94(typename std::messages<T1>::string_type &&a0,
    typename std::messages<T1>::string_type::size_type a1,
    typename std::messages<T1>::string_type::size_type a2,
    const std::allocator<T1> &a3) {
  return typename std::messages<T1>::string_type(std::move(a0), a1, a2, a3);
}
