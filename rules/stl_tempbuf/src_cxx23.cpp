#include <algorithm>
#include <utility>

template <typename T1> std::pair<T1 *, std::ptrdiff_t> f1(std::ptrdiff_t a0) {
  return std::get_temporary_buffer<T1>(a0);
}

template <typename T1> void f2(T1 *a0) {
  return std::return_temporary_buffer(a0);
}
