#include <functional>
#include <type_traits>
#include <utility>

template <typename T1> using t1 = std::_Bind<T1>;

template <typename T1, typename... T2> using t3 = std::_Bind_front<T1, T2...>;

template <typename T1, typename... T2, typename T3, typename... T4>
std::_Bind_front<T1, T2...> f3(int a0, T3 &&a1, T4 &&...a2) {
  return std::_Bind_front<T1, T2...>(a0, std::forward<T3>(a1),
                                     std::forward<T4>(a2)...);
}

template <typename T1, typename... T2, typename T3, typename... T4>
auto f4(std::_Bind_front<T1, T2...> &a0, T3 &&a1, T4 &&...a2) {
  return a0.template operator()<T3, T4...>(std::forward<T3>(a1),
                                           std::forward<T4>(a2)...);
}

template <typename T1> using t5 = std::_Mem_fn<T1>;

template <typename T1> using t6 = std::_Not_fn<T1>;

template <typename T1> std::_Not_fn<T1> f5(const std::_Not_fn<T1> &a0) {
  return std::_Not_fn<T1>(a0);
}

template <typename T1> std::_Not_fn<T1> f6(std::_Not_fn<T1> &&a0) {
  return std::_Not_fn<T1>(std::move(a0));
}

template <typename T1, typename T2> std::_Not_fn<T1> f7(T2 &&a0, int a1) {
  return std::_Not_fn<T1>(std::forward<T2>(a0), a1);
}

template <typename T1, typename... T2>
auto f8(std::_Not_fn<T1> &a0, T2 &&...a1) {
  return a0.template operator()<T2...>(std::forward<T2>(a1)...);
}

template <typename T1, typename... T2>
auto f9(const std::_Not_fn<T1> &a0, T2 &&...a1) {
  return a0.template operator()<T2...>(std::forward<T2>(a1)...);
}

template <typename T1, typename... T2>
auto f10(std::_Not_fn<T1> &&a0, T2 &&...a1) {
  return std::move(a0).template operator()<T2...>(std::forward<T2>(a1)...);
}

template <typename T1, typename... T2>
auto f11(const std::_Not_fn<T1> &&a0, T2 &&...a1) {
  return std::move(a0).template operator()<T2...>(std::forward<T2>(a1)...);
}

template <typename T1, typename... T2> auto f12(T1 &&a0, T2 &&...a1) {
  return std::bind(std::forward<T1>(a0), std::forward<T2>(a1)...);
}

template <typename T1, typename... T2> auto f15(T1 &&a0, T2 &&...a1) {
  return std::bind_front(std::forward<T1>(a0), std::forward<T2>(a1)...);
}

template <typename T1, typename... T2>
std::invoke_result_t<T1, T2...> f16(T1 &&a0, T2 &&...a1) {
  return std::invoke(std::forward<T1>(a0), std::forward<T2>(a1)...);
}

template <typename T1, typename T2> auto f20(T1 T2::*a0) {
  return std::mem_fn(std::forward<T1 T2::*>(a0));
}

template <typename T1> auto f21(T1 &&a0) {
  return std::not_fn(std::forward<T1>(a0));
}

const auto &f22() { return std::placeholders::_1; }

const auto &f23() { return std::placeholders::_10; }

const auto &f24() { return std::placeholders::_11; }

const auto &f25() { return std::placeholders::_12; }

const auto &f26() { return std::placeholders::_13; }

const auto &f27() { return std::placeholders::_14; }

const auto &f28() { return std::placeholders::_15; }

const auto &f29() { return std::placeholders::_16; }

const auto &f30() { return std::placeholders::_17; }

const auto &f31() { return std::placeholders::_18; }

const auto &f32() { return std::placeholders::_19; }

const auto &f33() { return std::placeholders::_2; }

const auto &f34() { return std::placeholders::_20; }

const auto &f35() { return std::placeholders::_21; }

const auto &f36() { return std::placeholders::_22; }

const auto &f37() { return std::placeholders::_23; }

const auto &f38() { return std::placeholders::_24; }

const auto &f39() { return std::placeholders::_25; }

const auto &f40() { return std::placeholders::_26; }

const auto &f41() { return std::placeholders::_27; }

const auto &f42() { return std::placeholders::_28; }

const auto &f43() { return std::placeholders::_29; }

const auto &f44() { return std::placeholders::_3; }

const auto &f45() { return std::placeholders::_4; }

const auto &f46() { return std::placeholders::_5; }

const auto &f47() { return std::placeholders::_6; }

const auto &f48() { return std::placeholders::_7; }

const auto &f49() { return std::placeholders::_8; }

const auto &f50() { return std::placeholders::_9; }
