#include <iterator>
#include <utility>

using t25 = std::bidirectional_iterator_tag;

std::bidirectional_iterator_tag f1() {
  return std::bidirectional_iterator_tag();
}

std::bidirectional_iterator_tag f2(const std::bidirectional_iterator_tag &a0) {
  return std::bidirectional_iterator_tag(a0);
}

std::bidirectional_iterator_tag f3(std::bidirectional_iterator_tag &&a0) {
  return std::bidirectional_iterator_tag(std::move(a0));
}

std::bidirectional_iterator_tag &f4(std::bidirectional_iterator_tag &a0,
                                    const std::bidirectional_iterator_tag &a1) {
  return a0.operator=(a1);
}

std::bidirectional_iterator_tag &f5(std::bidirectional_iterator_tag &a0,
                                    std::bidirectional_iterator_tag &&a1) {
  return a0.operator=(std::move(a1));
}

using t26 = std::forward_iterator_tag;

std::forward_iterator_tag f6() { return std::forward_iterator_tag(); }

std::forward_iterator_tag f7(const std::forward_iterator_tag &a0) {
  return std::forward_iterator_tag(a0);
}

std::forward_iterator_tag f8(std::forward_iterator_tag &&a0) {
  return std::forward_iterator_tag(std::move(a0));
}

std::forward_iterator_tag &f9(std::forward_iterator_tag &a0,
                              const std::forward_iterator_tag &a1) {
  return a0.operator=(a1);
}

std::forward_iterator_tag &f10(std::forward_iterator_tag &a0,
                               std::forward_iterator_tag &&a1) {
  return a0.operator=(std::move(a1));
}

using t27 = std::input_iterator_tag;

std::input_iterator_tag f11() { return std::input_iterator_tag(); }

std::input_iterator_tag f12(const std::input_iterator_tag &a0) {
  return std::input_iterator_tag(a0);
}

std::input_iterator_tag f13(std::input_iterator_tag &&a0) {
  return std::input_iterator_tag(std::move(a0));
}

std::input_iterator_tag &f14(std::input_iterator_tag &a0,
                             const std::input_iterator_tag &a1) {
  return a0.operator=(a1);
}

std::input_iterator_tag &f15(std::input_iterator_tag &a0,
                             std::input_iterator_tag &&a1) {
  return a0.operator=(std::move(a1));
}

using t28 = std::output_iterator_tag;

std::output_iterator_tag f16() { return std::output_iterator_tag(); }

std::output_iterator_tag f17(const std::output_iterator_tag &a0) {
  return std::output_iterator_tag(a0);
}

std::output_iterator_tag f18(std::output_iterator_tag &&a0) {
  return std::output_iterator_tag(std::move(a0));
}

std::output_iterator_tag &f19(std::output_iterator_tag &a0,
                              const std::output_iterator_tag &a1) {
  return a0.operator=(a1);
}

std::output_iterator_tag &f20(std::output_iterator_tag &a0,
                              std::output_iterator_tag &&a1) {
  return a0.operator=(std::move(a1));
}

using t29 = std::random_access_iterator_tag;

std::random_access_iterator_tag f21() {
  return std::random_access_iterator_tag();
}

std::random_access_iterator_tag f22(const std::random_access_iterator_tag &a0) {
  return std::random_access_iterator_tag(a0);
}

std::random_access_iterator_tag f23(std::random_access_iterator_tag &&a0) {
  return std::random_access_iterator_tag(std::move(a0));
}

std::random_access_iterator_tag &
f24(std::random_access_iterator_tag &a0,
    const std::random_access_iterator_tag &a1) {
  return a0.operator=(a1);
}

std::random_access_iterator_tag &f25(std::random_access_iterator_tag &a0,
                                     std::random_access_iterator_tag &&a1) {
  return a0.operator=(std::move(a1));
}
