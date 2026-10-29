#include <algorithm>
#include <utility>

template <typename T1> std::back_insert_iterator<T1> f1(T1 &a0) {
  return std::back_inserter(a0);
}

template <typename T1> using t1 = std::const_iterator<T1>;

template <typename T1> using t2 = std::const_sentinel<T1>;

template <typename T1> std::front_insert_iterator<T1> f2(T1 &a0) {
  return std::front_inserter(a0);
}

template <typename T1>
std::insert_iterator<T1> f3(T1 &a0, std::ranges::iterator_t<T1> a1) {
  return std::inserter(a0, std::forward<std::ranges::iterator_t<T1>>(a1));
}

template <typename T1> using t3 = std::iter_const_reference_t<T1>;

template <typename T1>
std::const_iterator<T1> f4(T1 a0)
  requires(std::input_iterator<T1>)
{
  return std::make_const_iterator(std::move(a0));
}

template <typename T1>
std::const_sentinel<T1> f5(T1 a0)
  requires(std::semiregular<T1>)
{
  return std::make_const_sentinel(std::move(a0));
}

template <typename T1> std::move_iterator<T1> f6(T1 a0) {
  return std::make_move_iterator(std::move(a0));
}

template <typename T1> std::reverse_iterator<T1> f7(T1 a0) {
  return std::make_reverse_iterator(std::move(a0));
}

template <typename T1, typename T2>
bool f8(const std::reverse_iterator<T1> &a0,
        const std::reverse_iterator<T2> &a1) {
  return std::operator!=(a0, a1);
}

template <typename T1>
std::move_iterator<T1> f9(typename std::move_iterator<T1>::difference_type a0,
                          const std::move_iterator<T1> &a1) {
  return std::operator+(
      std::forward<typename std::move_iterator<T1>::difference_type>(a0), a1);
}

template <typename T1>
std::reverse_iterator<T1>
f10(typename std::reverse_iterator<T1>::difference_type a0,
    const std::reverse_iterator<T1> &a1) {
  return std::operator+(
      std::forward<typename std::reverse_iterator<T1>::difference_type>(a0),
      a1);
}

template <typename T1, typename T2>
auto f11(const std::move_iterator<T1> &a0, const std::move_iterator<T2> &a1) {
  return std::operator-(a0, a1);
}

template <typename T1, typename T2>
auto f12(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T2> &a1) {
  return std::operator-(a0, a1);
}

template <typename T1, typename T2>
bool f13(const std::move_iterator<T1> &a0, const std::move_iterator<T2> &a1) {
  return std::operator<(a0, a1);
}

template <typename T1, typename T2>
bool f14(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T2> &a1) {
  return std::operator<(a0, a1);
}

template <typename T1, typename T2>
bool f15(const std::move_iterator<T1> &a0, const std::move_iterator<T2> &a1) {
  return std::operator<=(a0, a1);
}

template <typename T1, typename T2>
bool f16(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T2> &a1) {
  return std::operator<=(a0, a1);
}

template <typename T1>
std::compare_three_way_result_t<T1> f17(const std::move_iterator<T1> &a0,
                                        const std::move_iterator<T1> &a1)
  requires(std::three_way_comparable<T1>)
{
  return std::operator<=>(a0, a1);
}

template <typename T1>
std::compare_three_way_result_t<T1, T1> f18(const std::reverse_iterator<T1> &a0,
                                            const std::reverse_iterator<T1> &a1)
  requires(std::three_way_comparable<T1>)
{
  return std::operator<=>(a0, a1);
}

template <typename T1, typename T2>
std::compare_three_way_result_t<T1, T2> f19(const std::move_iterator<T1> &a0,
                                            const std::move_iterator<T2> &a1)
  requires(std::three_way_comparable_with<T2, T1>)
{
  return std::operator<=>(a0, a1);
}

template <typename T1, typename T2>
std::compare_three_way_result_t<T1, T2> f20(const std::reverse_iterator<T1> &a0,
                                            const std::reverse_iterator<T2> &a1)
  requires(std::three_way_comparable_with<T2, T1>)
{
  return std::operator<=>(a0, a1);
}

template <typename T1>
bool f21(const std::move_iterator<T1> &a0, const std::move_iterator<T1> &a1) {
  return std::operator==(a0, a1);
}

template <typename T1>
bool f22(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T1> &a1) {
  return std::operator==(a0, a1);
}

template <typename T1, typename T2>
bool f23(const std::move_iterator<T1> &a0, const std::move_iterator<T2> &a1) {
  return std::operator==(a0, a1);
}

template <typename T1, typename T2>
bool f24(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T2> &a1) {
  return std::operator==(a0, a1);
}

template <typename T1, typename T2>
bool f25(const std::move_iterator<T1> &a0, const std::move_iterator<T2> &a1) {
  return std::operator>(a0, a1);
}

template <typename T1, typename T2>
bool f26(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T2> &a1) {
  return std::operator>(a0, a1);
}

template <typename T1, typename T2>
bool f27(const std::move_iterator<T1> &a0, const std::move_iterator<T2> &a1) {
  return std::operator>=(a0, a1);
}

template <typename T1, typename T2>
bool f28(const std::reverse_iterator<T1> &a0,
         const std::reverse_iterator<T2> &a1) {
  return std::operator>=(a0, a1);
}
