#include <iterator>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1>
std::insert_iterator<T1> f7(T1 &a0, typename T1::iterator a1) {
  return std::inserter<T1>(a0, a1);
}
