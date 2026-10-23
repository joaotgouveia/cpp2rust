#include <array>
#include <tuple>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2> T1 f91(T2 &&a0) {
  return std::make_from_tuple<T1, T2>(std::forward<T2>(a0));
}
