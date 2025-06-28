#include "./putils.hpp"

#include <format>
#include <functional>
#include <string>

#include "../utils.hpp"
#include "./shared.hpp"
#include "utils.hpp"

using namespace json::parsers::shared;

void json::parsers::utils::assert_has_member(const JSONValue &value,
                                             const char *key,
                                             const std::string &path) {
    if (!value.HasMember(key)) {
        throw ParsingError(std::format("{}.{} is not found", path, key),
                           ParsingErrorType::NO_MEMBER);
    }
};

void json::parsers::utils::assert_member_is_valid(const std::string &key,
                                                  const bool &isValid,
                                                  const std::string &path) {
    if (!isValid) {
        throw ParsingError(std::format("{}.{} is invalid", path, key),
                           ParsingErrorType::INVALID_MEMBER);
    };
};

unsigned int json::parsers::utils::extractUint(const JSONValue &value,
                                               const char *key,
                                               const std::string &path) {
    return json::parsers::utils::extractValue(
        value, key, path, [](const JSONValue &v) -> bool { return v.IsUint(); },
        FuncFromLambda([](const JSONValue &v) -> unsigned int { return v.GetUint(); }));
};
