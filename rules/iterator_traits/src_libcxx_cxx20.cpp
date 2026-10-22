#include "../common/init.h"
#include "../common/traits.h"
#include <iterator>
#include <utility>

using t29 = std::contiguous_iterator_tag;

std::contiguous_iterator_tag f26() { return std::contiguous_iterator_tag(); }

std::contiguous_iterator_tag f27(const std::contiguous_iterator_tag &a0) {
  return std::contiguous_iterator_tag(a0);
}

std::contiguous_iterator_tag f28(std::contiguous_iterator_tag &&a0) {
  return std::contiguous_iterator_tag(std::move(a0));
}

std::contiguous_iterator_tag &f29(std::contiguous_iterator_tag &a0,
                                  const std::contiguous_iterator_tag &a1) {
  return a0.operator=(a1);
}

std::contiguous_iterator_tag &f30(std::contiguous_iterator_tag &a0,
                                  std::contiguous_iterator_tag &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> using t30 = std::iter_reference_t<T1>;

template <typename T1> using t31 = std::iter_value_t<T1>;
