#include <forward_list>
#include <initializer_list>
#include <utility>

template <typename T1, typename T2>
void f100(std::forward_list<T1, T2> &a0,
          const typename std::forward_list<T1, T2>::value_type &a1) {
  return a0.remove(a1);
}
