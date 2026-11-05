// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

#include "converter/rules/matcher.h"

#include <cassert>
#include <cctype>

#include "converter/converter.h"

namespace cpp2rust::Matcher {

std::string MapBinding(Converter &converter, const Bindings &bindings,
                       unsigned n) {
  return converter.GetTemplateArgumentAsString(bindings.at(n));
}

// Substitutes template arguments into a target template string. Each template
// parameter in `tgt_template` is replaced with its corresponding argument from
// `bindings`, which is either a type or a value.
//
// Example:
//   bindings     = { int, 4 }
//   tgt_template = "[T1; T2]"
//   result       = "[i32; 4_usize]"
std::string InstantiateTgt(Converter &converter, const Bindings &bindings,
                           const std::string &tgt_template) {
  assert(bindings.size() <= TranslationRule::kMaxGenerics &&
         "template placeholder exceeds kMaxGenerics");
  std::string instantiated_template = tgt_template;
  std::string::size_type pos = 0;
  while ((pos = instantiated_template.find('T', pos)) != std::string::npos) {
    if (pos + 1 >= instantiated_template.size()) {
      break;
    }
    if (!std::isdigit(instantiated_template[pos + 1])) {
      ++pos;
      continue;
    }
    auto end = pos + 1;
    unsigned n = 0;
    while (end < instantiated_template.size() &&
           std::isdigit(instantiated_template[end])) {
      n = n * 10 + (instantiated_template[end++] - '0');
    }
    auto repl = MapBinding(converter, bindings, n - 1);
    instantiated_template.replace(pos, end - pos, repl);
    pos += repl.length();
  }
  return instantiated_template;
}

// A pointer to a generic is the converter's pointer to its binding when the
// binding changes the pointer's kind: a function type makes it the function
// pointer itself, as functions are not objects (Ptr<T1> of void() is
// FnPtr<fn()>), and a const one makes a raw pointer *const.
std::string InstantiateType(Converter &converter, const Bindings &bindings,
                            const TranslationRule::TypeInfo &info) {
  if (auto n = info.pointee_generic; n && n <= bindings.size() &&
      bindings[n - 1].getKind() == clang::TemplateArgument::Type) {
    auto pointee = bindings[n - 1].getAsType();
    if (pointee->isFunctionType() ||
        (info.is_unsafe_pointer && pointee.isConstQualified())) {
      auto &ctx = converter.GetASTContext();
      return converter.GetTemplateArgumentAsString(
          clang::TemplateArgument(ctx.getPointerType(pointee)));
    }
  }
  return InstantiateTgt(converter, bindings, info.type);
}

} // namespace cpp2rust::Matcher
