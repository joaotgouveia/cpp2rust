#include <algorithm>
#include <barrier>
#include <utility>

template <typename T1> using t1 = std::barrier<T1>;

template <typename T1> using t2 = typename std::barrier<T1>::arrival_token;

template <typename T1> std::ptrdiff_t f1() { return std::barrier<T1>::max(); }

template <typename T1> std::barrier<T1> f2(std::ptrdiff_t a0, T1 a1) {
  return std::barrier<T1>(a0, std::move(a1));
}

template <typename T1>
typename std::barrier<T1>::arrival_token f3(std::barrier<T1> &a0,
                                            std::ptrdiff_t a1) {
  return a0.arrive(a1);
}

template <typename T1>
void f4(const std::barrier<T1> &a0,
        typename std::barrier<T1>::arrival_token &&a1) {
  return a0.wait(std::move(a1));
}

template <typename T1> void f5(std::barrier<T1> &a0) {
  return a0.arrive_and_wait();
}

template <typename T1> void f6(std::barrier<T1> &a0) {
  return a0.arrive_and_drop();
}

template <typename T1>
typename std::barrier<T1>::arrival_token
f7(typename std::barrier<T1>::arrival_token &&a0) {
  return typename std::barrier<T1>::arrival_token(std::move(a0));
}

template <typename T1>
typename std::barrier<T1>::arrival_token &
f8(typename std::barrier<T1>::arrival_token &a0,
   typename std::barrier<T1>::arrival_token &&a1) {
  return a0.operator=(std::move(a1));
}
