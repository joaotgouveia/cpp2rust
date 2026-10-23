#include <queue>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2, typename T3>
using t1 = std::priority_queue<T1, T2, T3>;

template <typename T1, typename T2, typename T3>
using t2 = typename std::priority_queue<T1, T2, T3>::value_type;

template <typename T1, typename T2, typename T3>
using t3 = typename std::priority_queue<T1, T2, T3>::reference;

template <typename T1, typename T2, typename T3>
using t4 = typename std::priority_queue<T1, T2, T3>::const_reference;

template <typename T1, typename T2, typename T3>
using t5 = typename std::priority_queue<T1, T2, T3>::size_type;

template <typename T1, typename T2, typename T3>
using t6 = typename std::priority_queue<T1, T2, T3>::container_type;

template <typename T1, typename T2, typename T3>
using t7 = typename std::priority_queue<T1, T2, T3>::value_compare;

template <typename T1, typename T2, typename T3>
std::priority_queue<T1, T2, T3> f1(const std::priority_queue<T1, T2, T3> &a0) {
  return std::priority_queue<T1, T2, T3>(a0);
}

template <typename T1, typename T2, typename T3>
std::priority_queue<T1, T2, T3> &f2(std::priority_queue<T1, T2, T3> &a0,
                                    const std::priority_queue<T1, T2, T3> &a1) {
  return a0.operator=(a1);
}

template <typename T1, typename T2, typename T3>
std::priority_queue<T1, T2, T3> f3(std::priority_queue<T1, T2, T3> &&a0) {
  return std::priority_queue<T1, T2, T3>(std::move(a0));
}

template <typename T1, typename T2, typename T3>
std::priority_queue<T1, T2, T3> &f4(std::priority_queue<T1, T2, T3> &a0,
                                    std::priority_queue<T1, T2, T3> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1, typename T2, typename T3>
bool f5(const std::priority_queue<T1, T2, T3> &a0) {
  return a0.empty();
}

template <typename T1, typename T2, typename T3>
typename std::priority_queue<T1, T2, T3>::size_type
f6(const std::priority_queue<T1, T2, T3> &a0) {
  return a0.size();
}

template <typename T1, typename T2, typename T3>
typename std::priority_queue<T1, T2, T3>::const_reference
f7(const std::priority_queue<T1, T2, T3> &a0) {
  return a0.top();
}

template <typename T1, typename T2, typename T3>
void f8(std::priority_queue<T1, T2, T3> &a0,
        const typename std::priority_queue<T1, T2, T3>::value_type &a1) {
  return a0.push(a1);
}

template <typename T1, typename T2, typename T3>
void f9(std::priority_queue<T1, T2, T3> &a0,
        typename std::priority_queue<T1, T2, T3>::value_type &&a1) {
  return a0.push(std::move(a1));
}

template <typename T1, typename T2, typename T3>
void f10(std::priority_queue<T1, T2, T3> &a0) {
  return a0.pop();
}

template <typename T1, typename T2, typename T3>
void f11(std::priority_queue<T1, T2, T3> &a0,
         std::priority_queue<T1, T2, T3> &a1) {
  return a0.swap(a1);
}

template <typename T1, typename T2, typename T3, typename... T4>
void f12(std::priority_queue<T1, T2, T3> &a0, T4 &&...a1) {
  return a0.template emplace<T4...>(std::forward<T4>(a1)...);
}
