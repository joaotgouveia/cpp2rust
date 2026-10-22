#include "../common/init.h"
#include "../common/traits.h"
#include <future>
#include <utility>

bool f1(std::thread::id a0, std::thread::id a1) {
  return std::move(a0) == std::move(a1);
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

template <
    typename T1, typename... T2,
    typename T3 = typename std::enable_if<
        std::integral_constant<
            bool, std::integral_constant<
                      bool, !std::is_same<typename std::remove_cv<
                                              typename std::remove_reference<
                                                  T1>::type>::type,
                                          std::thread>::value>::value>::value,
        void>::type>
std::thread f15(T1 &&a0, T2 &&...a1) {
  return std::thread(std::forward<T1>(a0), std::forward<T2>(a1)...);
}
