#include <future>
#include <utility>

template <typename T1, typename... T2> using t1 = std::packaged_task<T1(T2...)>;

template <typename T1, typename... T2> std::packaged_task<T1(T2...)> f1() {
  return std::packaged_task<T1(T2...)>();
}

template <typename T1, typename... T2>
std::packaged_task<T1(T2...)> f2(std::packaged_task<T1(T2...)> &&a0) {
  return std::packaged_task<T1(T2...)>(std::move(a0));
}

template <typename T1, typename... T2>
std::packaged_task<T1(T2...)> &f3(std::packaged_task<T1(T2...)> &a0,
                                  std::packaged_task<T1(T2...)> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename... T2>
void f4(std::packaged_task<T1(T2...)> &a0, std::packaged_task<T1(T2...)> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename... T2>
bool f5(const std::packaged_task<T1(T2...)> &a0) {
  return a0.valid();
}

template <typename T1, typename... T2>
std::future<T1> f6(std::packaged_task<T1(T2...)> &a0) {
  return a0.get_future();
}

template <typename T1, typename... T2>
void f7(std::packaged_task<T1(T2...)> &a0, T2... a1) {
  return a0.operator()(std::move(a1)...);
}

template <typename T1, typename... T2>
void f8(std::packaged_task<T1(T2...)> &a0, T2... a1) {
  return a0.make_ready_at_thread_exit(std::move(a1)...);
}

template <typename T1, typename... T2>
void f9(std::packaged_task<T1(T2...)> &a0) {
  return a0.reset();
}

template <typename T1, typename... T2, typename T3>
std::packaged_task<T1(T2...)> f10(T3 &&a0)
  requires(!(std::is_same<T3, const std::packaged_task<T1(T2...)> &>::value) &&
           !(std::is_same<T3, std::packaged_task<T1(T2...)>>::value))
{
  return std::packaged_task<T1(T2...)>(std::forward<T3>(a0));
}
