#include <type_traits>
#include <utility>

template <typename T1> using t1 = std::has_unique_object_representations<T1>;

template <typename T1>
bool f1(const std::has_unique_object_representations<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1>
bool f2(const std::has_unique_object_representations<T1> &a0) {
  return a0.operator()();
}
