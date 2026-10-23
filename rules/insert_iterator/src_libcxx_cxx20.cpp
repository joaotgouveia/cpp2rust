#include <iterator>
#include <ranges>
#include <utility>

template <typename T1>
std::insert_iterator<T1> f8(T1 &a0, std::ranges::iterator_t<T1> a1) {
  return std::insert_iterator<T1>(a0, a1);
}

template <typename T1>
std::insert_iterator<T1> f9(T1 &a0, std::ranges::iterator_t<T1> a1) {
  return std::inserter<T1>(a0, a1);
}
