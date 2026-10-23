#include <ios>
#include <string>
#include <utility>

template <typename T1> long f6(const std::collate<T1> &a0) {
  return a0.use_count();
}
