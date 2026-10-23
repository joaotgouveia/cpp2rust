#include <ios>
#include <iosfwd>
#include <sstream>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2, typename T3>
std::basic_stringbuf<T1, T2, T3>
f28(std::basic_string<typename std::basic_stringbuf<T1, T2, T3>::char_type, T2,
                      T3> &&a0,
    std::ios_base::openmode a1) {
  return std::basic_stringbuf<T1, T2, T3>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<typename std::basic_stringbuf<T1, T2, T3>::char_type, T2, T3>
f29(std::basic_stringbuf<T1, T2, T3> &&a0) {
  return std::move(a0).str();
}

template <typename T1, typename T2, typename T3>
void f30(std::basic_stringbuf<T1, T2, T3> &a0,
         std::basic_string<typename std::basic_stringbuf<T1, T2, T3>::char_type,
                           T2, T3> &&a1) {
  return a0.str(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_stringbuf<T1, T2, T3>
f31(const std::basic_string<T1, T2, T4> &a0,
    const typename std::basic_stringbuf<T1, T2, T3>::allocator_type &a1) {
  return std::basic_stringbuf<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_stringbuf<T1, T2, T3>
f32(const std::basic_string<T1, T2, T4> &a0, std::ios_base::openmode a1,
    const typename std::basic_stringbuf<T1, T2, T3>::allocator_type &a2) {
  return std::basic_stringbuf<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_stringbuf<T1, T2, T3> f33(const std::basic_string<T1, T2, T4> &a0,
                                     std::ios_base::openmode a1) {
  return std::basic_stringbuf<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<T1, T2, T4> f34(const std::basic_stringbuf<T1, T2, T3> &a0,
                                  const T4 &a1) {
  return a0.template str<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
void f35(std::basic_stringbuf<T1, T2, T3> &a0,
         const std::basic_string<T1, T2, T4> &a1) {
  return a0.template str<T4>(a1);
}
