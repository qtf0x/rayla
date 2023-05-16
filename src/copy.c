#include "copy.h"

void *copy(void *destination, const void *source, size_t num)
{
	for (size_t i = 0; i < num; ++i) {
		char *d = destination + i;
		const char *s = source + i;
		*d = *s;
	}

	return destination;
}
