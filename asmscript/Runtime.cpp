#include "Runtime.h"

#include <cstdio>
#include <cinttypes>

void RtPrint(int64_t value)
{
	printf("%" PRId64, value);
}

void RtPrint(const char* text, size_t length)
{
	fwrite(text, 1, length, stdout);
}
