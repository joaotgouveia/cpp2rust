#include <list>
#include <utility>

template <typename T1, typename T2>
bool f6(const std::list<T1, T2> &a0, const std::list<T1, T2> &a1) {
  return std::operator== <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
void f7(std::list<T1, T2> &a0, std::list<T1, T2> &a1) {
  return std::swap<T1, T2>(a0, a1);
}
