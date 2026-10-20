#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <utility>

using t1 = std::incrementable_traits<unsigned __int128>;

using t2 = std::incrementable_traits<unsigned __int128>::difference_type;

std::incrementable_traits<unsigned __int128> f1() {
  return std::incrementable_traits<unsigned __int128>();
}

std::incrementable_traits<unsigned __int128>
f2(const std::incrementable_traits<unsigned __int128> &a0) {
  return std::incrementable_traits<unsigned __int128>(a0);
}

std::incrementable_traits<unsigned __int128>
f3(std::incrementable_traits<unsigned __int128> &&a0) {
  return std::incrementable_traits<unsigned __int128>(std::move(a0));
}

std::incrementable_traits<unsigned __int128> &
f4(std::incrementable_traits<unsigned __int128> &a0,
   const std::incrementable_traits<unsigned __int128> &a1) {
  return a0.operator=(a1);
}

std::incrementable_traits<unsigned __int128> &
f5(std::incrementable_traits<unsigned __int128> &a0,
   std::incrementable_traits<unsigned __int128> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> using t3 = std::incrementable_traits<T1 *>;

template <typename T1>
using t4 = typename std::incrementable_traits<T1 *>::difference_type;

template <typename T1> using t5 = std::incrementable_traits<T1>;

template <typename T1>
using t6 = typename std::incrementable_traits<T1>::difference_type;

template <typename T1, typename T2>
using t7 = std::incrementable_traits<std::common_iterator<T1, T2>>;

template <typename T1, typename T2>
using t8 = typename std::incrementable_traits<
    std::common_iterator<T1, T2>>::difference_type;
