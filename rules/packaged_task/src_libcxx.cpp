#include <future>
#include <memory>
#include <utility>

template <typename... T1> using t2 = std::packaged_task<void(T1...)>;

template <typename... T1> std::packaged_task<void(T1...)> f15() {
  return std::packaged_task<void(T1...)>();
}

template <typename... T1>
std::packaged_task<void(T1...)> f16(std::packaged_task<void(T1...)> &&a0) {
  return std::packaged_task<void(T1...)>(std::move(a0));
}

template <typename... T1>
std::packaged_task<void(T1...)> &f17(std::packaged_task<void(T1...)> &a0,
                                     std::packaged_task<void(T1...)> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename... T1>
void f18(std::packaged_task<void(T1...)> &a0,
         std::packaged_task<void(T1...)> &a1) {
  return a0.swap(a1);
}

template <typename... T1> bool f19(const std::packaged_task<void(T1...)> &a0) {
  return a0.valid();
}

template <typename... T1>
std::future<void> f20(std::packaged_task<void(T1...)> &a0) {
  return a0.get_future();
}

template <typename... T1>
void f21(std::packaged_task<void(T1...)> &a0, T1... a1) {
  return a0.operator()(std::move(a1)...);
}

template <typename... T1>
void f22(std::packaged_task<void(T1...)> &a0, T1... a1) {
  return a0.make_ready_at_thread_exit(std::move(a1)...);
}

template <typename... T1> void f23(std::packaged_task<void(T1...)> &a0) {
  return a0.reset();
}
