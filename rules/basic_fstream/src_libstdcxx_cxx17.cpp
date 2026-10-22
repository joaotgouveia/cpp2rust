#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <fstream>
#include <ios>
#include <string>
#include <type_traits>
#include <utility>

template <
    typename T1, typename T2, typename T3,
    typename T4 = std::enable_if_t<
        std::is_same_v<
            T3, decltype(std::declval<T3 &>().make_preferred().filename())>,
        T3>>
std::basic_fstream<T1, T2>
f107(const T3 &a0, typename std::basic_fstream<T1, T2>::openmode a1) {
  return std::basic_fstream<T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3>
std::enable_if_t<
    std::is_same_v<T3,
                   decltype(std::declval<T3 &>().make_preferred().filename())>,
    void>
f108(std::basic_fstream<T1, T2> &a0, const T3 &a1,
     typename std::basic_fstream<T1, T2>::openmode a2) {
  return a0.template open<T3>(a1, a2);
}

template <typename T1, typename T2>
std::basic_fstream<T1, T2> &f109(std::basic_fstream<T1, T2> &a0,
                                 std::nullptr_t a1) {
  return a0.operator<<(a1);
}
