#include "../common/init.h"
#include "../common/traits.h"
#include <iterator>
#include <utility>

template <typename T1, typename T2> void f1(T1 &a0, T2 a1) {
  return std::advance<T1, T2>(a0, a1);
}
