#include <algorithm>
#include <initializer_list>
#include <ios>
#include <iosfwd>
#include <locale>
#include <string>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1>
bool f82(const typename std::messages<T1>::string_type &a0, T1 a1) {
  return a0.contains(a1);
}

template <typename T1>
bool f83(const typename std::messages<T1>::string_type &a0, T1 const *a1) {
  return a0.contains(a1);
}

template <typename T1, typename T2>
typename std::messages<T1>::string_type f84(std::from_range_t a0, T2 &&a1,
                                            const std::allocator<T1> &a2) {
  return typename std::messages<T1>::string_type(std::move(a0),
                                                 std::forward<T2>(a1), a2);
}
