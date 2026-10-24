#include <istream>
#include <random>
#include <utility>

template <typename T1, std::size_t T2, typename T3, typename T4,
          typename T5 = std::__detail::_If_seed_seq_for<
              T4, std::independent_bits_engine<T1, T2, T3>, T3>>
std::independent_bits_engine<T1, T2, T3> f15(T4 &a0)
  requires(!(std::is_same<T4, const T1>::value))
{
  return std::independent_bits_engine<T1, T2, T3>(a0);
}
