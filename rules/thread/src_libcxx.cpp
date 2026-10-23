#include <ostream>
#include <thread>
#include <utility>

void f23(std::thread &a0, std::thread &a1) { return std::swap(a0, a1); }

using t4 = std::thread;

using t5 = std::thread::id;

using t6 = std::thread::native_handle_type;

std::thread f24() { return std::thread(); }

std::thread f25(std::thread &&a0) { return std::thread(std::move(a0)); }

std::thread &f26(std::thread &a0, std::thread &&a1) {
  return a0.operator=(std::move(a1));
}

void f27(std::thread &a0, std::thread &a1) { return a0.swap(a1); }

bool f28(const std::thread &a0) { return a0.joinable(); }

void f29(std::thread &a0) { return a0.join(); }

void f30(std::thread &a0) { return a0.detach(); }

std::thread::id f31(const std::thread &a0) { return a0.get_id(); }

std::thread::native_handle_type f32(std::thread &a0) {
  return a0.native_handle();
}

unsigned int f33() { return std::thread::hardware_concurrency(); }

template <typename T1, typename... T2,
          typename std::enable_if<
              !std::is_same<typename std::remove_cv<
                                typename std::remove_reference<T1>::type>::type,
                            std::thread>::value,
              int>::type T3 = 0>
std::thread f34(T1 &&a0, T2 &&...a1) {
  return std::thread(std::forward<T1>(a0), std::forward<T2>(a1)...);
}

std::thread::id f35() { return std::thread::id(); }

std::thread::id f36(const std::thread::id &a0) { return std::thread::id(a0); }

std::thread::id f37(std::thread::id &&a0) {
  return std::thread::id(std::move(a0));
}

std::thread::id &f38(std::thread::id &a0, const std::thread::id &a1) {
  return a0.operator=(a1);
}

std::thread::id &f39(std::thread::id &a0, std::thread::id &&a1) {
  return a0.operator=(std::move(a1));
}
