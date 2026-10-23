#include <condition_variable>
#include <utility>

template <typename T1, typename T2,
          typename T3 = typename std::enable_if<std::is_constructible<
              std::__weak_ptr<T1>, const std::shared_ptr<T2> &>::value>::type>
std::weak_ptr<T1> f10(const std::shared_ptr<T2> &a0) {
  return std::weak_ptr<T1>(a0);
}
