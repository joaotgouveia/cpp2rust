#include "../common/init.h"
#include "../common/traits.h"
#include <ios>
#include <iosfwd>
#include <ostream>
#include <syncstream>
#include <utility>

template <typename T1, typename T2, typename T3>
std::basic_ostream<typename std::basic_osyncstream<T1, T2, T3>::char_type,
                   typename std::basic_osyncstream<T1, T2, T3>::traits_type> *
f68(const std::basic_osyncstream<T1, T2, T3> &a0) {
  return a0.tie();
}

template <typename T1, typename T2, typename T3>
std::basic_ostream<typename std::basic_osyncstream<T1, T2, T3>::char_type,
                   typename std::basic_osyncstream<T1, T2, T3>::traits_type> *
f69(std::basic_osyncstream<T1, T2, T3> &a0,
    std::basic_ostream<typename std::basic_osyncstream<T1, T2, T3>::char_type,
                       typename std::basic_osyncstream<T1, T2, T3>::traits_type>
        *a1) {
  return a0.tie(a1);
}
