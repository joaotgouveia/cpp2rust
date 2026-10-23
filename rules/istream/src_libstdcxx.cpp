#include <istream>
#include <utility>

template <typename T1, typename T2> T1 &&f11(T1 &&a0, T2 &&a1) {
  return std::operator>> <T1, T2>(std::forward<T1>(a0), std::forward<T2>(a1));
}
