#include <algorithm>
#include <utility>

using t6 = std::contiguous_iterator_tag;

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
