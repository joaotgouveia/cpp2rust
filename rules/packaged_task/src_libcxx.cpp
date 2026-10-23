#include <future>
#include <memory>
#include <utility>

template <typename... T1> using t2 = std::packaged_task<void(T1...)>;

template <typename... T1> std::packaged_task<void(T1...)> f14() {
  return std::packaged_task<void(T1...)>();
}

template <typename... T1>
std::packaged_task<void(T1...)> f15(std::packaged_task<void(T1...)> &&a0) {
  return std::packaged_task<void(T1...)>(std::move(a0));
}

template <typename... T1>
std::packaged_task<void(T1...)> &f16(std::packaged_task<void(T1...)> &a0,
                                     std::packaged_task<void(T1...)> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename... T1>
void f17(std::packaged_task<void(T1...)> &a0,
         std::packaged_task<void(T1...)> &a1) {
  return a0.swap(a1);
}

template <typename... T1> bool f18(const std::packaged_task<void(T1...)> &a0) {
  return a0.valid();
}

template <typename... T1>
std::future<void> f19(std::packaged_task<void(T1...)> &a0) {
  return a0.get_future();
}

template <typename... T1>
void f20(std::packaged_task<void(T1...)> &a0, T1... a1) {
  return a0.operator()(a1...);
}

template <typename... T1>
void f21(std::packaged_task<void(T1...)> &a0, T1... a1) {
  return a0.make_ready_at_thread_exit(a1...);
}

template <typename... T1> void f22(std::packaged_task<void(T1...)> &a0) {
  return a0.reset();
}

template <
    typename... T1, typename T2,
    std::__enable_if_t<!std::is_same<std::__remove_cvref_t<T2>,
                                     std::packaged_task<void(T1...)>>::value,
                       int>
        T3 = 0>
std::packaged_task<void(T1...)> f23(T2 &&a0) {
  return std::packaged_task<void(T1...)>(std::forward<T2>(a0));
}

template <
    typename T1, typename... T2, typename T3,
    std::__enable_if_t<!std::is_same<std::__remove_cvref_t<T3>,
                                     std::packaged_task<T1(T2...)>>::value,
                       int>
        T4 = 0>
std::packaged_task<T1(T2...)> f24(T3 &&a0) {
  return std::packaged_task<T1(T2...)>(std::forward<T3>(a0));
}
