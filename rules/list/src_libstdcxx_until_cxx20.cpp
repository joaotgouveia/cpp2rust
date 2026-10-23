#include <initializer_list>
#include <list>
#include <utility>

template <typename T1, typename T2>
void f82(std::list<T1, T2> &a0, const T1 &a1) {
  return a0.remove(a1);
}
