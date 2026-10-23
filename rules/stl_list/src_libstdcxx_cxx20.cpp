#include <list>
#include <utility>

template <typename T1, typename T2>
auto f8(const std::list<T1, T2> &a0, const std::list<T1, T2> &a1) {
  return std::operator<=> <T1, T2>(a0, a1);
}
