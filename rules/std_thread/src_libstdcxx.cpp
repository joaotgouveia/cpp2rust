#include <future>
#include <utility>

bool f1(std::thread::id a0, std::thread::id a1) {
  return std::operator==(std::move(a0), std::move(a1));
}

void f2(std::thread &a0, std::thread &a1) { return std::swap(a0, a1); }

std::thread::id f3() { return std::this_thread::get_id(); }

void f4() { return std::this_thread::yield(); }

using t1 = std::thread;

using t2 = std::thread::native_handle_type;

using t3 = std::thread::id;

std::thread f5() { return std::thread(); }

std::thread f6(std::thread &&a0) { return std::thread(std::move(a0)); }

std::thread &f7(std::thread &a0, std::thread &&a1) {
  return a0.operator=(std::move(a1));
}

void f8(std::thread &a0, std::thread &a1) { return a0.swap(a1); }

bool f9(const std::thread &a0) { return a0.joinable(); }

void f10(std::thread &a0) { return a0.join(); }

void f11(std::thread &a0) { return a0.detach(); }

std::thread::id f12(const std::thread &a0) { return a0.get_id(); }

std::thread::native_handle_type f13(std::thread &a0) {
  return a0.native_handle();
}

unsigned int f14() { return std::thread::hardware_concurrency(); }

template <typename T1, typename... T2,
          typename T3 = std::_Require<std::__not_<
              std::is_same<std::__remove_cvref_t<T1>, std::thread>>>>
std::thread f15(T1 &&a0, T2 &&...a1) {
  return std::thread(std::forward<T1>(a0), std::forward<T2>(a1)...);
}

std::thread::id f16() { return std::thread::id(); }

std::thread::id f17(std::thread::native_handle_type a0) {
  return std::thread::id(a0);
}

std::thread::id f18(const std::thread::id &a0) { return std::thread::id(a0); }

std::thread::id f19(std::thread::id &&a0) {
  return std::thread::id(std::move(a0));
}

std::thread::id &f20(std::thread::id &a0, const std::thread::id &a1) {
  return a0.operator=(a1);
}

std::thread::id &f21(std::thread::id &a0, std::thread::id &&a1) {
  return a0.operator=(std::move(a1));
}
