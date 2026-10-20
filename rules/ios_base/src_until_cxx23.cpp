#include "../common/init.h"
#include "../common/traits.h"
#include <ios>
#include <iosfwd>
#include <string>
#include <utility>

std::ios_base::fmtflags f76(std::ios_base::fmtflags a0,
                            std::ios_base::fmtflags a1) {
  return a0 & a1;
}

std::ios_base::iostate f77(std::ios_base::iostate a0,
                           std::ios_base::iostate a1) {
  return a0 & a1;
}

std::ios_base::openmode f78(std::ios_base::openmode a0,
                            std::ios_base::openmode a1) {
  return a0 & a1;
}

const std::ios_base::fmtflags &f79(std::ios_base::fmtflags &a0,
                                   std::ios_base::fmtflags a1) {
  return std::operator&=(a0, a1);
}

const std::ios_base::iostate &f80(std::ios_base::iostate &a0,
                                  std::ios_base::iostate a1) {
  return std::operator&=(a0, a1);
}

const std::ios_base::openmode &f81(std::ios_base::openmode &a0,
                                   std::ios_base::openmode a1) {
  return std::operator&=(a0, a1);
}

std::ios_base::fmtflags f82(std::ios_base::fmtflags a0,
                            std::ios_base::fmtflags a1) {
  return a0 ^ a1;
}

std::ios_base::iostate f83(std::ios_base::iostate a0,
                           std::ios_base::iostate a1) {
  return a0 ^ a1;
}

std::ios_base::openmode f84(std::ios_base::openmode a0,
                            std::ios_base::openmode a1) {
  return a0 ^ a1;
}

const std::ios_base::fmtflags &f85(std::ios_base::fmtflags &a0,
                                   std::ios_base::fmtflags a1) {
  return std::operator^=(a0, a1);
}

const std::ios_base::iostate &f86(std::ios_base::iostate &a0,
                                  std::ios_base::iostate a1) {
  return std::operator^=(a0, a1);
}

const std::ios_base::openmode &f87(std::ios_base::openmode &a0,
                                   std::ios_base::openmode a1) {
  return std::operator^=(a0, a1);
}

std::ios_base::fmtflags f88(std::ios_base::fmtflags a0,
                            std::ios_base::fmtflags a1) {
  return a0 | a1;
}

std::ios_base::iostate f89(std::ios_base::iostate a0,
                           std::ios_base::iostate a1) {
  return a0 | a1;
}

std::ios_base::openmode f90(std::ios_base::openmode a0,
                            std::ios_base::openmode a1) {
  return a0 | a1;
}

const std::ios_base::fmtflags &f91(std::ios_base::fmtflags &a0,
                                   std::ios_base::fmtflags a1) {
  return std::operator|=(a0, a1);
}

const std::ios_base::iostate &f92(std::ios_base::iostate &a0,
                                  std::ios_base::iostate a1) {
  return std::operator|=(a0, a1);
}

const std::ios_base::openmode &f93(std::ios_base::openmode &a0,
                                   std::ios_base::openmode a1) {
  return std::operator|=(a0, a1);
}

std::ios_base::fmtflags f94(std::ios_base::fmtflags a0) { return ~a0; }

std::ios_base::iostate f95(std::ios_base::iostate a0) { return ~a0; }

std::ios_base::openmode f96(std::ios_base::openmode a0) { return ~a0; }
