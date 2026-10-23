#include <forward_list>
#include <initializer_list>
#include <utility>

template <typename T1, typename T2>
void f80(std::forward_list<T1, T2> &a0, const T1 &a1) {
  return a0.remove(a1);
}
