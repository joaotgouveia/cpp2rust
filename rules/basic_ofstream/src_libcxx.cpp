#include <fstream>
#include <ios>
#include <ostream>
#include <string.h>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2>
std::basic_ostream<typename std::basic_ofstream<T1, T2>::char_type,
                   typename std::basic_ofstream<T1, T2>::traits_type> *
f73(const std::basic_ofstream<T1, T2> &a0) {
  return a0.tie();
}

template <typename T1, typename T2>
std::basic_ostream<typename std::basic_ofstream<T1, T2>::char_type,
                   typename std::basic_ofstream<T1, T2>::traits_type> *
f74(std::basic_ofstream<T1, T2> &a0,
    std::basic_ostream<typename std::basic_ofstream<T1, T2>::char_type,
                       typename std::basic_ofstream<T1, T2>::traits_type> *a1) {
  return a0.tie(a1);
}
