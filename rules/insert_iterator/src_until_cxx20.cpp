#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <utility>

template <typename T1>
std::insert_iterator<T1> f6(T1 &a0, typename T1::iterator a1) {
  return std::insert_iterator<T1>(a0, a1);
}
