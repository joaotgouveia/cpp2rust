#include "../common/init.h"
#include "../common/traits.h"
#include <list>
#include <utility>

template <typename T1, typename T2>
bool f1(const std::list<T1, T2> &a0, const std::list<T1, T2> &a1) {
  return a0 == a1;
}

template <typename T1, typename T2>
void f2(std::list<T1, T2> &a0, std::list<T1, T2> &a1) {
  return std::swap<T1, T2>(a0, a1);
}
