#include <barrier>
#include <utility>

template <typename T1> using t1 = std::binder1st<T1>;

template <typename T1>
std::binder1st<T1> f1(const T1 &a0,
                      const typename T1::first_argument_type &a1) {
  return std::binder1st<T1>(a0, a1);
}

template <typename T1>
typename T1::result_type f2(const std::binder1st<T1> &a0,
                            const typename T1::second_argument_type &a1) {
  return a0.operator()(a1);
}

template <typename T1>
typename T1::result_type f3(const std::binder1st<T1> &a0,
                            typename T1::second_argument_type &a1) {
  return a0.operator()(a1);
}
