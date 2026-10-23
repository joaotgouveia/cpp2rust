#include <barrier>
#include <ostream>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f15(std::basic_ostream<T1, T2> &a0) {
  return std::emit_on_flush<T1, T2>(a0);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f16(std::basic_ostream<T1, T2> &a0) {
  return std::flush_emit<T1, T2>(a0);
}

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f17(std::basic_ostream<T1, T2> &a0) {
  return std::noemit_on_flush<T1, T2>(a0);
}
