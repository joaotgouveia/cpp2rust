#include <algorithm>
#include <ios>
#include <string>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1>
typename std::numpunct_byname<T1>::string_type
f71(T1 const *a0, const std::allocator<T1> &a1) {
  return typename std::numpunct_byname<T1>::string_type(a0, a1);
}

template <typename T1>
typename std::numpunct_byname<T1>::string_type
f72(typename std::numpunct_byname<T1>::string_type::size_type a0, T1 a1,
    const std::allocator<T1> &a2) {
  return typename std::numpunct_byname<T1>::string_type(a0, a1, a2);
}
