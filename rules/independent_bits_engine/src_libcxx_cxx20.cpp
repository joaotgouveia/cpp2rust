#include <istream>
#include <ostream>
#include <random>
#include <utility>

template <
    typename T1, std::size_t T2, typename T3, typename T4,
    std::__enable_if_t<std::__is_seed_sequence_v<
                           T4, std::independent_bits_engine<T1, T2, T3>> &&
                           !std::is_convertible<T4, T1>::value,
                       int>
        T5 = 0>
std::independent_bits_engine<T1, T2, T3> f22(T4 &a0)
  requires(!(std::is_same<T4, const T1>::value))
{
  return std::independent_bits_engine<T1, T2, T3>(a0);
}
