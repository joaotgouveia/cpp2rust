#include <memory>
#include <utility>

template <typename T1>
std::auto_ptr<T1> f19(typename std::auto_ptr<T1>::element_type *a0) {
  return std::auto_ptr<T1>(a0);
}
