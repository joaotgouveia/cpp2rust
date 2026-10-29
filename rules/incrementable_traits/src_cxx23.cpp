#include <algorithm>
#include <utility>

using t1 = std::incrementable_traits<__int128>;

using t2 = std::incrementable_traits<__int128>::difference_type;

std::incrementable_traits<__int128> f1() {
  return std::incrementable_traits<__int128>();
}

std::incrementable_traits<__int128>
f2(const std::incrementable_traits<__int128> &a0) {
  return std::incrementable_traits<__int128>(a0);
}

std::incrementable_traits<__int128>
f3(std::incrementable_traits<__int128> &&a0) {
  return std::incrementable_traits<__int128>(std::move(a0));
}

std::incrementable_traits<__int128> &
f4(std::incrementable_traits<__int128> &a0,
   const std::incrementable_traits<__int128> &a1) {
  return a0.operator=(a1);
}

std::incrementable_traits<__int128> &
f5(std::incrementable_traits<__int128> &a0,
   std::incrementable_traits<__int128> &&a1) {
  return a0.operator=(std::move(a1));
}

using t3 = std::incrementable_traits<unsigned __int128>;

using t4 = std::incrementable_traits<unsigned __int128>::difference_type;

std::incrementable_traits<unsigned __int128> f6() {
  return std::incrementable_traits<unsigned __int128>();
}

std::incrementable_traits<unsigned __int128>
f7(const std::incrementable_traits<unsigned __int128> &a0) {
  return std::incrementable_traits<unsigned __int128>(a0);
}

std::incrementable_traits<unsigned __int128>
f8(std::incrementable_traits<unsigned __int128> &&a0) {
  return std::incrementable_traits<unsigned __int128>(std::move(a0));
}

std::incrementable_traits<unsigned __int128> &
f9(std::incrementable_traits<unsigned __int128> &a0,
   const std::incrementable_traits<unsigned __int128> &a1) {
  return a0.operator=(a1);
}

std::incrementable_traits<unsigned __int128> &
f10(std::incrementable_traits<unsigned __int128> &a0,
    std::incrementable_traits<unsigned __int128> &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1>
  requires(std::is_object_v<T1>)
using t5 = std::incrementable_traits<T1 *>;

template <typename T1>
using t6 = typename std::incrementable_traits<T1 *>::difference_type;

template <typename T1> using t7 = std::incrementable_traits<const T1>;

template <typename T1> using t8 = std::incrementable_traits<T1>;

template <typename T1>
using t9 = typename std::incrementable_traits<T1>::difference_type;

template <typename T1, typename T2>
using t10 = std::incrementable_traits<std::common_iterator<T1, T2>>;

template <typename T1, typename T2>
using t11 = typename std::incrementable_traits<
    std::common_iterator<T1, T2>>::difference_type;
