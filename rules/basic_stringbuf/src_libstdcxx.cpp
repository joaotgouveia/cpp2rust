#include <sstream>
#include <string>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2, typename T3>
std::basic_stringbuf<T1, T2, T3>
f25(const std::basic_string<
        typename std::basic_stringbuf<T1, T2, T3>::char_type, T2, T3> &a0,
    std::ios_base::openmode a1) {
  return std::basic_stringbuf<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::basic_string<typename std::basic_stringbuf<T1, T2, T3>::char_type, T2, T3>
f26(const std::basic_stringbuf<T1, T2, T3> &a0) {
  return a0.str();
}

template <typename T1, typename T2, typename T3>
void f27(
    std::basic_stringbuf<T1, T2, T3> &a0,
    const std::basic_string<
        typename std::basic_stringbuf<T1, T2, T3>::char_type, T2, T3> &a1) {
  return a0.str(a1);
}
