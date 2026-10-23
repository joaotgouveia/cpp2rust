#include <algorithm>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

using t7 = std::incrementable_traits<unsigned __int128>;

using t8 = std::incrementable_traits<unsigned __int128>::difference_type;

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
