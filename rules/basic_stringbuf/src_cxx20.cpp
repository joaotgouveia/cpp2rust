#include <ios>
#include <iosfwd>
#include <sstream>
#include <string>
#include <string_view>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2, typename T3>
std::basic_stringbuf<T1, T2, T3>
f20(const typename std::basic_stringbuf<T1, T2, T3>::allocator_type &a0) {
  return std::basic_stringbuf<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
std::basic_stringbuf<T1, T2, T3>
f21(std::ios_base::openmode a0,
    const typename std::basic_stringbuf<T1, T2, T3>::allocator_type &a1) {
  return std::basic_stringbuf<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_stringbuf<T1, T2, T3>
f22(std::basic_stringbuf<T1, T2, T3> &&a0,
    const typename std::basic_stringbuf<T1, T2, T3>::allocator_type &a1) {
  return std::basic_stringbuf<T1, T2, T3>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_stringbuf<T1, T2, T3>::allocator_type
f23(const std::basic_stringbuf<T1, T2, T3> &a0) {
  return a0.get_allocator();
}

template <typename T1, typename T2, typename T3>
std::basic_string_view<typename std::basic_stringbuf<T1, T2, T3>::char_type,
                       typename std::basic_stringbuf<T1, T2, T3>::traits_type>
f24(const std::basic_stringbuf<T1, T2, T3> &a0) {
  return a0.view();
}
