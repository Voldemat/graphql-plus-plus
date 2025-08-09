#pragma once

#include <filesystem>
#include <functional>
#include <string>

namespace cli {
void run_config_action(
    const std::string &configPath,
    const std::function<void(const std::string &, const std::filesystem::path &,
                             const std::string &, const std::string &)> &jsonStringCallback);
};
