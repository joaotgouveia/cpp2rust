#include "../common/init.h"
#include "../common/traits.h"
#include <future>
#include <tuple>
#include <utility>

template <typename T1, typename... T2, typename T3,
          typename T4 = typename std::enable_if<!std::is_same<
              std::packaged_task<T1(T2...)>,
              typename std::remove_cv<typename std::remove_reference<
                  T3>::type>::type>::value>::type>
std::packaged_task<T1(T2...)> f10(T3 &&a0) {
  return std::packaged_task<T1(T2...)>(std::forward<T3>(a0));
}
