#include <condition_variable>
#include <utility>

template <typename T1, typename T2,
          typename T3 = typename std::enable_if<std::is_constructible<
              std::__weak_ptr<T1>, const std::shared_ptr<T2> &>::value>::type>
std::weak_ptr<T1> f10(const std::shared_ptr<T2> &a0) {
  return std::weak_ptr<T1>(a0);
}

template <typename T1, typename T2>
typename std::enable_if<std::is_assignable<std::__weak_ptr<T1> &,
                                           const std::shared_ptr<T2> &>::value,
                        std::weak_ptr<T1> &>::type
f11(std::weak_ptr<T1> &a0, const std::shared_ptr<T2> &a1) {
  return a0.template operator= <T2>(a1);
}

template <typename T1, typename T2>
bool f12(const std::weak_ptr<T1> &a0,
         const std::__shared_ptr<T2, (std::_Lock_policy)2U> &a1) {
  return a0.template owner_before<T2>(a1);
}
