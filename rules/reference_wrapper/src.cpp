#include "../common/init.h"
#include "../common/traits.h"
#include <functional>
#include <string>
#include <utility>

template <typename T1> using t1 = std::reference_wrapper<T1>;

template <typename T1> using t2 = typename std::reference_wrapper<T1>::type;

template <typename T1, typename T2> std::reference_wrapper<T1> f1(T2 &&a0) {
  return std::reference_wrapper<T1>(std::forward<T2>(a0));
}
