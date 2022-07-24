#pragma once

#include "Error.h"

#include <memory>
#include <string>
#include <unordered_map>
#include <vector>

struct Statement;

[[nodiscard]] Error Compile(std::unordered_map<std::string, std::vector<std::unique_ptr<Statement>>>& procedures, std::basic_string<unsigned char>& code, size_t& entry);
