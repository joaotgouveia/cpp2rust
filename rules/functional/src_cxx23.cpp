#include <functional>
#include <type_traits>
#include <utility>

template <typename T1> using t1 = std::_Bind<T1>;

template <typename T1, typename... T2> using t2 = std::_Bind_back<T1, T2...>;

template <typename T1, typename... T2, typename T3, typename... T4>
std::_Bind_back<T1, T2...> f1(int a0, T3 &&a1, T4 &&...a2) {
  return std::_Bind_back<T1, T2...>(a0, std::forward<T3>(a1),
                                    std::forward<T4>(a2)...);
}

template <typename T1, typename... T2, typename T3, typename... T4>
auto f2(std::_Bind_back<T1, T2...> &a0, T3 &&a1, T4 &&...a2) {
  return a0.template operator()<T3, T4...>(std::forward<T3>(a1),
                                           std::forward<T4>(a2)...);
}

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

template <typename T1, typename T2> using t4 = std::_Bind_result<T1, T2>;

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

template <int T1> using t7 = std::_Placeholder<T1>;

template <typename T1, typename... T2> auto f12(T1 &&a0, T2 &&...a1) {
  return std::bind(std::forward<T1>(a0), std::forward<T2>(a1)...);
}

template <typename T1, typename T2, typename... T3>
auto f13(T2 &&a0, T3 &&...a1) {
  return std::bind<T1>(std::forward<T2>(a0), std::forward<T3>(a1)...);
}

template <typename T1, typename... T2>
std::_Bind_back<std::decay_t<T1>, std::decay_t<T2>...> f14(T1 &&a0,
                                                           T2 &&...a1) {
  return std::bind_back(std::forward<T1>(a0), std::forward<T2>(a1)...);
}

template <typename T1, typename... T2>
std::_Bind_front<std::decay_t<T1>, std::decay_t<T2>...> f15(T1 &&a0,
                                                            T2 &&...a1) {
  return std::bind_front(std::forward<T1>(a0), std::forward<T2>(a1)...);
}

template <typename T1, typename... T2>
std::invoke_result_t<T1, T2...> f16(T1 &&a0, T2 &&...a1) {
  return std::invoke(std::forward<T1>(a0), std::forward<T2>(a1)...);
}

template <typename T1, typename T2, typename... T3>
T1 f17(T2 &&a0, T3 &&...a1)
  requires(std::is_invocable_r_v<T1, T2, T3...>)
{
  return std::invoke_r<T1>(std::forward<T2>(a0), std::forward<T3>(a1)...);
}

template <typename T1> bool f18() { return std::is_bind_expression_v<T1>; }

template <typename T1> int f19() { return std::is_placeholder_v<T1>; }

template <typename T1, typename T2> std::_Mem_fn<T1 T2::*> f20(T1 T2::*a0) {
  return std::mem_fn(std::forward<T1 T2::*>(a0));
}

template <typename T1> auto f21(T1 &&a0) {
  return std::not_fn(std::forward<T1>(a0));
}

const std::_Placeholder<1> &f22() { return std::placeholders::_1; }

const std::_Placeholder<10> &f23() { return std::placeholders::_10; }

const std::_Placeholder<11> &f24() { return std::placeholders::_11; }

const std::_Placeholder<12> &f25() { return std::placeholders::_12; }

const std::_Placeholder<13> &f26() { return std::placeholders::_13; }

const std::_Placeholder<14> &f27() { return std::placeholders::_14; }

const std::_Placeholder<15> &f28() { return std::placeholders::_15; }

const std::_Placeholder<16> &f29() { return std::placeholders::_16; }

const std::_Placeholder<17> &f30() { return std::placeholders::_17; }

const std::_Placeholder<18> &f31() { return std::placeholders::_18; }

const std::_Placeholder<19> &f32() { return std::placeholders::_19; }

const std::_Placeholder<2> &f33() { return std::placeholders::_2; }

const std::_Placeholder<20> &f34() { return std::placeholders::_20; }

const std::_Placeholder<21> &f35() { return std::placeholders::_21; }

const std::_Placeholder<22> &f36() { return std::placeholders::_22; }

const std::_Placeholder<23> &f37() { return std::placeholders::_23; }

const std::_Placeholder<24> &f38() { return std::placeholders::_24; }

const std::_Placeholder<25> &f39() { return std::placeholders::_25; }

const std::_Placeholder<26> &f40() { return std::placeholders::_26; }

const std::_Placeholder<27> &f41() { return std::placeholders::_27; }

const std::_Placeholder<28> &f42() { return std::placeholders::_28; }

const std::_Placeholder<29> &f43() { return std::placeholders::_29; }

const std::_Placeholder<3> &f44() { return std::placeholders::_3; }

const std::_Placeholder<4> &f45() { return std::placeholders::_4; }

const std::_Placeholder<5> &f46() { return std::placeholders::_5; }

const std::_Placeholder<6> &f47() { return std::placeholders::_6; }

const std::_Placeholder<7> &f48() { return std::placeholders::_7; }

const std::_Placeholder<8> &f49() { return std::placeholders::_8; }

const std::_Placeholder<9> &f50() { return std::placeholders::_9; }
