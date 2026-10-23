#include <sstream>
#include <string>
#include <string_view>
#include <utility>

template <typename T1, typename T2, typename T3>
std::basic_stringbuf<T1, T2, T3>
f40(typename std::basic_stringbuf<T1, T2, T3>::string_type &&a0,
    std::ios_base::openmode a1) {
  return std::basic_stringbuf<T1, T2, T3>(std::move(a0), a1);
}

template <typename T1, typename T2, typename T3>
typename std::basic_stringbuf<T1, T2, T3>::string_type
f41(std::basic_stringbuf<T1, T2, T3> &&a0) {
  return std::move(a0).str();
}

template <typename T1, typename T2, typename T3>
void f42(std::basic_stringbuf<T1, T2, T3> &a0,
         typename std::basic_stringbuf<T1, T2, T3>::string_type &&a1) {
  return a0.str(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_stringbuf<T1, T2, T3>
f43(const std::basic_string<
        typename std::basic_stringbuf<T1, T2, T3>::char_type,
        typename std::basic_stringbuf<T1, T2, T3>::traits_type, T4> &a0,
    const typename std::basic_stringbuf<T1, T2, T3>::allocator_type &a1) {
  return std::basic_stringbuf<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_stringbuf<T1, T2, T3>
f44(const std::basic_string<
        typename std::basic_stringbuf<T1, T2, T3>::char_type,
        typename std::basic_stringbuf<T1, T2, T3>::traits_type, T4> &a0,
    std::ios_base::openmode a1,
    const typename std::basic_stringbuf<T1, T2, T3>::allocator_type &a2) {
  return std::basic_stringbuf<T1, T2, T3>(a0, a1, a2);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_stringbuf<T1, T2, T3>
f45(const std::basic_string<
        typename std::basic_stringbuf<T1, T2, T3>::char_type,
        typename std::basic_stringbuf<T1, T2, T3>::traits_type, T4> &a0,
    std::ios_base::openmode a1) {
  return std::basic_stringbuf<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::basic_string<typename std::basic_stringbuf<T1, T2, T3>::char_type,
                  typename std::basic_stringbuf<T1, T2, T3>::traits_type, T4>
f46(const std::basic_stringbuf<T1, T2, T3> &a0, const T4 &a1) {
  return a0.template str<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
void f47(std::basic_stringbuf<T1, T2, T3> &a0,
         const std::basic_string<
             typename std::basic_stringbuf<T1, T2, T3>::char_type,
             typename std::basic_stringbuf<T1, T2, T3>::traits_type, T4> &a1) {
  return a0.template str<T4>(a1);
}
