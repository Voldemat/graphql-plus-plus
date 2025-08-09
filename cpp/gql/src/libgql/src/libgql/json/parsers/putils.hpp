#pragma once

#include <functional>
#include <string>

#include "../utils.hpp"

namespace gql::json::parsers::utils {
void assert_has_member(const JSONValue &value, const char *key,
                       const std::string &path);

void assert_member_is_valid(const std::string &key, const bool &isValid,
                            const std::string &path);

template <typename T>
T extractValue(const JSONValue &value, const char *key, const std::string &path,
               const std::function<bool(const JSONValue &)> &validator,
               const std::function<T(const JSONValue &)> &extractor) {
    assert_has_member(value, key, path);
    assert_member_is_valid(key, validator(value[key]), path);
    return extractor(value[key]);
};

unsigned int extractUint(const JSONValue &value, const char *key,
                         const std::string &path);
};  // namespace gql::json::parsers::utils
