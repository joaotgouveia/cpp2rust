#include <algorithm>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1> std::reverse_iterator<T1> f33(T1 a0) {
  return std::make_reverse_iterator<T1>(a0);
}
