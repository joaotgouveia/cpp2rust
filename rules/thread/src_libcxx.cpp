#include <ostream>
#include <thread>
#include <utility>

void f23(std::thread &a0, std::thread &a1) { return std::swap(a0, a1); }

template <typename T1, typename T2>
std::basic_ostream<T1, T2> &f24(std::basic_ostream<T1, T2> &a0,
                                std::__thread_id a1) {
  return std::operator<< <T1, T2>(a0, std::move(a1));
}

using t4 = std::thread;

using t5 = std::thread::id;

using t6 = std::thread::native_handle_type;

std::thread f25() { return std::thread(); }

std::thread f26(std::thread &&a0) { return std::thread(std::move(a0)); }

std::thread &f27(std::thread &a0, std::thread &&a1) {
  return a0.operator=(std::move(a1));
}

void f28(std::thread &a0, std::thread &a1) { return a0.swap(a1); }

bool f29(const std::thread &a0) { return a0.joinable(); }

void f30(std::thread &a0) { return a0.join(); }

void f31(std::thread &a0) { return a0.detach(); }

std::thread::id f32(const std::thread &a0) { return a0.get_id(); }

std::thread::native_handle_type f33(std::thread &a0) {
  return a0.native_handle();
}

unsigned int f34() { return std::thread::hardware_concurrency(); }

template <typename T1, typename... T2,
          std::__enable_if_t<
              !std::is_same<std::__remove_cvref_t<T1>, std::thread>::value, int>
              T3 = 0>
std::thread f35(T1 &&a0, T2 &&...a1) {
  return std::thread(std::forward<T1>(a0), std::forward<T2>(a1)...);
}
