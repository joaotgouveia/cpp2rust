#include <syncstream>
#include <utility>

template <typename T1, typename T2, typename T3>
void f1(std::basic_syncbuf<T1, T2, T3> &a0,
        std::basic_syncbuf<T1, T2, T3> &a1) {
  return std::swap<T1, T2, T3>(a0, a1);
}
