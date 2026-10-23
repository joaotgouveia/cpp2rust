#include <stdexcept>
#include <string>
#include <utility>

std::domain_error f46(std::domain_error &&a0) {
  return std::domain_error(std::move(a0));
}

std::domain_error &f47(std::domain_error &a0, std::domain_error &&a1) {
  return a0.operator=(std::move(a1));
}

std::invalid_argument f48(std::invalid_argument &&a0) {
  return std::invalid_argument(std::move(a0));
}

std::invalid_argument &f49(std::invalid_argument &a0,
                           std::invalid_argument &&a1) {
  return a0.operator=(std::move(a1));
}

std::length_error f50(std::length_error &&a0) {
  return std::length_error(std::move(a0));
}

std::length_error &f51(std::length_error &a0, std::length_error &&a1) {
  return a0.operator=(std::move(a1));
}

std::logic_error f52(std::logic_error &&a0) {
  return std::logic_error(std::move(a0));
}

std::logic_error &f53(std::logic_error &a0, std::logic_error &&a1) {
  return a0.operator=(std::move(a1));
}

std::out_of_range f54(std::out_of_range &&a0) {
  return std::out_of_range(std::move(a0));
}

std::out_of_range &f55(std::out_of_range &a0, std::out_of_range &&a1) {
  return a0.operator=(std::move(a1));
}

std::overflow_error f56(std::overflow_error &&a0) {
  return std::overflow_error(std::move(a0));
}

std::overflow_error &f57(std::overflow_error &a0, std::overflow_error &&a1) {
  return a0.operator=(std::move(a1));
}

std::range_error f58(std::range_error &&a0) {
  return std::range_error(std::move(a0));
}

std::range_error &f59(std::range_error &a0, std::range_error &&a1) {
  return a0.operator=(std::move(a1));
}

std::runtime_error f60(std::runtime_error &&a0) {
  return std::runtime_error(std::move(a0));
}

std::runtime_error &f61(std::runtime_error &a0, std::runtime_error &&a1) {
  return a0.operator=(std::move(a1));
}

std::underflow_error f62(std::underflow_error &&a0) {
  return std::underflow_error(std::move(a0));
}

std::underflow_error &f63(std::underflow_error &a0, std::underflow_error &&a1) {
  return a0.operator=(std::move(a1));
}
