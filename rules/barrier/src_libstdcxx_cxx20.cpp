#include <algorithm>
#include <barrier>
#include <utility>

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
