#include "../common/init.h"
#include "../common/traits.h"
#include <ios>
#include <utility>

template <typename T1>
int f85(const std::collate<T1> &a0,
        const typename std::collate<T1>::char_type *a1,
        const typename std::collate<T1>::char_type *a2,
        const typename std::collate<T1>::char_type *a3,
        const typename std::collate<T1>::char_type *a4) {
  return a0.compare(a1, a2, a3, a4);
}

template <typename T1>
typename std::collate<T1>::string_type
f86(const std::collate<T1> &a0, const typename std::collate<T1>::char_type *a1,
    const typename std::collate<T1>::char_type *a2) {
  return a0.transform(a1, a2);
}

template <typename T1>
long f87(const std::collate<T1> &a0,
         const typename std::collate<T1>::char_type *a1,
         const typename std::collate<T1>::char_type *a2) {
  return a0.hash(a1, a2);
}

template <typename T1> long f88(const std::collate<T1> &a0) {
  return a0.use_count();
}
