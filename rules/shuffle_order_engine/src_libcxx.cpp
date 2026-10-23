#include <cstddef>
#include <istream>
#include <ostream>
#include <random>
#include <utility>

template <typename T1, std::size_t T2>
bool f16(const std::shuffle_order_engine<T1, T2> &a0,
         const std::shuffle_order_engine<T1, T2> &a1) {
  return std::operator!= <T1, T2>(a0, a1);
}

template <typename T1, std::size_t T2>
bool f17(const std::shuffle_order_engine<T1, T2> &a0,
         const std::shuffle_order_engine<T1, T2> &a1) {
  return std::operator== <T1, T2>(a0, a1);
}

template <typename T1, typename T2, typename T3, std::size_t T4>
std::basic_ostream<T1, T2> &f18(std::basic_ostream<T1, T2> &a0,
                                const std::shuffle_order_engine<T3, T4> &a1) {
  return std::operator<< <T1, T2, T3, T4>(a0, a1);
}

template <typename T1, typename T2, typename T3, std::size_t T4>
std::basic_istream<T1, T2> &f19(std::basic_istream<T1, T2> &a0,
                                std::shuffle_order_engine<T3, T4> &a1) {
  return std::operator>> <T1, T2, T3, T4>(a0, a1);
}

template <
    typename T1, std::size_t T2, typename T3,
    std::__enable_if_t<
        std::__is_seed_sequence_v<T3, std::shuffle_order_engine<T1, T2>>, int>
        T4 = 0>
void f20(std::shuffle_order_engine<T1, T2> &a0, T3 &a1) {
  return a0.template seed<T3>(a1);
}
