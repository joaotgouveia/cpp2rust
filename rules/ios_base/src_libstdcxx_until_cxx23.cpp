#include <ios>
#include <iosfwd>
#include <string>
#include <utility>

std::ios_base::fmtflags f88(std::ios_base::fmtflags a0,
                            std::ios_base::fmtflags a1) {
  return std::operator&(a0, a1);
}

std::ios_base::iostate f89(std::ios_base::iostate a0,
                           std::ios_base::iostate a1) {
  return std::operator&(a0, a1);
}

std::ios_base::openmode f90(std::ios_base::openmode a0,
                            std::ios_base::openmode a1) {
  return std::operator&(a0, a1);
}

const std::ios_base::fmtflags &f91(std::ios_base::fmtflags &a0,
                                   std::ios_base::fmtflags a1) {
  return std::operator&=(a0, a1);
}

const std::ios_base::iostate &f92(std::ios_base::iostate &a0,
                                  std::ios_base::iostate a1) {
  return std::operator&=(a0, a1);
}

const std::ios_base::openmode &f93(std::ios_base::openmode &a0,
                                   std::ios_base::openmode a1) {
  return std::operator&=(a0, a1);
}

std::ios_base::fmtflags f94(std::ios_base::fmtflags a0,
                            std::ios_base::fmtflags a1) {
  return std::operator^(a0, a1);
}

std::ios_base::iostate f95(std::ios_base::iostate a0,
                           std::ios_base::iostate a1) {
  return std::operator^(a0, a1);
}

std::ios_base::openmode f96(std::ios_base::openmode a0,
                            std::ios_base::openmode a1) {
  return std::operator^(a0, a1);
}

const std::ios_base::fmtflags &f97(std::ios_base::fmtflags &a0,
                                   std::ios_base::fmtflags a1) {
  return std::operator^=(a0, a1);
}

const std::ios_base::iostate &f98(std::ios_base::iostate &a0,
                                  std::ios_base::iostate a1) {
  return std::operator^=(a0, a1);
}

const std::ios_base::openmode &f99(std::ios_base::openmode &a0,
                                   std::ios_base::openmode a1) {
  return std::operator^=(a0, a1);
}

std::ios_base::fmtflags f100(std::ios_base::fmtflags a0,
                             std::ios_base::fmtflags a1) {
  return std::operator|(a0, a1);
}

std::ios_base::iostate f101(std::ios_base::iostate a0,
                            std::ios_base::iostate a1) {
  return std::operator|(a0, a1);
}

std::ios_base::openmode f102(std::ios_base::openmode a0,
                             std::ios_base::openmode a1) {
  return std::operator|(a0, a1);
}

const std::ios_base::fmtflags &f103(std::ios_base::fmtflags &a0,
                                    std::ios_base::fmtflags a1) {
  return std::operator|=(a0, a1);
}

const std::ios_base::iostate &f104(std::ios_base::iostate &a0,
                                   std::ios_base::iostate a1) {
  return std::operator|=(a0, a1);
}

const std::ios_base::openmode &f105(std::ios_base::openmode &a0,
                                    std::ios_base::openmode a1) {
  return std::operator|=(a0, a1);
}

std::ios_base::fmtflags f106(std::ios_base::fmtflags a0) {
  return std::operator~(a0);
}

std::ios_base::iostate f107(std::ios_base::iostate a0) {
  return std::operator~(a0);
}

std::ios_base::openmode f108(std::ios_base::openmode a0) {
  return std::operator~(a0);
}
