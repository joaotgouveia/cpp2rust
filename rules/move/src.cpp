#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <utility>

template <typename T1> typename std::remove_reference<T1>::type &&f1(T1 &&a0) {
  return std::move<T1>(std::forward<T1>(a0));
}
