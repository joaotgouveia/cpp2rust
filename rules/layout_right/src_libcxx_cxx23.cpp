#include <mdspan>
#include <utility>

template <typename T1> using t1 = std::layout_right::mapping<T1>;

template <typename T1>
using t2 = typename std::layout_right::mapping<T1>::extents_type;

template <typename T1>
using t3 = typename std::layout_right::mapping<T1>::index_type;

template <typename T1>
using t4 = typename std::layout_right::mapping<T1>::size_type;

template <typename T1>
using t5 = typename std::layout_right::mapping<T1>::rank_type;

template <typename T1>
using t6 = typename std::layout_right::mapping<T1>::layout_type;

template <typename T1> std::layout_right::mapping<T1> f1() {
  return std::layout_right::mapping<T1>();
}

template <typename T1>
std::layout_right::mapping<T1> f2(const std::layout_right::mapping<T1> &a0) {
  return std::layout_right::mapping<T1>(a0);
}

template <typename T1> std::layout_right::mapping<T1> f3(const T1 &a0) {
  return std::layout_right::mapping<T1>(a0);
}

template <typename T1>
std::layout_right::mapping<T1> &f4(std::layout_right::mapping<T1> &a0,
                                   const std::layout_right::mapping<T1> &a1) {
  return a0.operator=(a1);
}

template <typename T1> const T1 &f5(const std::layout_right::mapping<T1> &a0) {
  return a0.extents();
}

template <typename T1>
typename std::layout_right::mapping<T1>::index_type
f6(const std::layout_right::mapping<T1> &a0) {
  return a0.required_span_size();
}

template <typename T1> bool f7() {
  return std::layout_right::mapping<T1>::is_always_unique();
}

template <typename T1> bool f8() {
  return std::layout_right::mapping<T1>::is_always_exhaustive();
}

template <typename T1> bool f9() {
  return std::layout_right::mapping<T1>::is_always_strided();
}

template <typename T1> bool f10() {
  return std::layout_right::mapping<T1>::is_unique();
}

template <typename T1> bool f11() {
  return std::layout_right::mapping<T1>::is_exhaustive();
}

template <typename T1> bool f12() {
  return std::layout_right::mapping<T1>::is_strided();
}

template <typename T1>
typename std::layout_right::mapping<T1>::index_type
f13(const std::layout_right::mapping<T1> &a0,
    typename std::layout_right::mapping<T1>::rank_type a1)
  requires((std::layout_right::mapping<T1>::extents_type::rank() > 0))
{
  return a0.stride(std::move(a1));
}

template <typename T1, typename T2>
std::layout_right::mapping<T1> f14(const std::layout_right::mapping<T2> &a0)
  requires((std::is_constructible_v<
              typename std::layout_right::mapping<T1>::extents_type, T2>)) &&
          (!(std::is_same<T2, T1>::value))
{
  return std::layout_right::mapping<T1>(a0);
}

template <typename T1, typename T2>
std::layout_right::mapping<T1> f15(const std::layout_left::mapping<T2> &a0)
  requires((std::is_constructible_v<
                typename std::layout_right::mapping<T1>::extents_type, T2> &&
            T2::rank() <= 1))
{
  return std::layout_right::mapping<T1>(a0);
}

template <typename T1, typename T2>
std::layout_right::mapping<T1> f16(const std::layout_stride::mapping<T2> &a0)
  requires((std::is_constructible_v<
            typename std::layout_right::mapping<T1>::extents_type, T2>))
{
  return std::layout_right::mapping<T1>(a0);
}

template <typename T1, typename... T2>
typename std::layout_right::mapping<T1>::index_type
f17(const std::layout_right::mapping<T1> &a0, T2... a1)
  requires(((sizeof...(T2) ==
             std::layout_right::mapping<T1>::extents_type::rank()) &&
            (std::is_convertible_v<
                 T2, typename std::layout_right::mapping<T1>::index_type> &&
             ...) &&
            (std::is_nothrow_constructible_v<
                 typename std::layout_right::mapping<T1>::index_type, T2> &&
             ...)))
{
  return a0.template operator()<T2...>(a1...);
}

template <typename T1, typename T2>
bool f18(const std::layout_right::mapping<T1> &a0,
         const std::layout_right::mapping<T2> &a1)
  requires((T2::rank() == std::layout_right::mapping<T1>::extents_type::rank()))
{
  return operator==(a0, a1);
}
