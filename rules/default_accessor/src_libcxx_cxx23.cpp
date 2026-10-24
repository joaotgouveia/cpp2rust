#include <cstddef>
#include <mdspan>
#include <utility>

template <typename T1> using t1 = std::default_accessor<T1>;

template <typename T1>
using t2 = typename std::default_accessor<T1>::offset_policy;

template <typename T1>
using t3 = typename std::default_accessor<T1>::element_type;

template <typename T1> using t4 = typename std::default_accessor<T1>::reference;

template <typename T1>
using t5 = typename std::default_accessor<T1>::data_handle_type;

template <typename T1> std::default_accessor<T1> f1() {
  return std::default_accessor<T1>();
}

template <typename T1>
T1 &f2(const std::default_accessor<T1> &a0, T1 *a1, std::size_t a2) {
  return a0.access(a1, a2);
}

template <typename T1>
T1 *f3(const std::default_accessor<T1> &a0, T1 *a1, std::size_t a2) {
  return a0.offset(a1, a2);
}

template <typename T1, typename T2>
std::default_accessor<T1> f4(std::default_accessor<T2> a0)
  requires((std::is_convertible_v<
            T2 (*)[], typename std::default_accessor<T1>::element_type (*)[]>))
{
  return std::default_accessor<T1>(std::move(a0));
}
