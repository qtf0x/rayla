/**
 * @file tuple.c
 * @author Vincent Marias ~ @qtf0x
 * @date 05/16/2023
 * 
 * @copyright MIT License Copyright (c) 2023 Vincent Marias
 * 
 * @brief Implementations for tuple.h functions.
 */

#include "tuple.h"
#include "utils.h" // for rlFeq

bool rlIsPoint(const struct RLtuple tup)
{
	return rlFeq(tup.w, 1.f);
}

bool rlIsVector(const struct RLtuple tup)
{
	return rlFeq(tup.w, 0.f);
}

bool rlTeq(const struct RLtuple a, const struct RLtuple b)
{
	return rlFeq(a.x, b.x) && rlFeq(a.y, b.y) && rlFeq(a.z, b.z) &&
	       rlFeq(a.w, b.w);
}

struct RLtuple rlMakeTuple(const float x, const float y, const float z,
			   const float w)
{
	struct RLtuple tup = { { x }, { y }, { z }, { w } };
	return tup;
}

struct RLtuple rlMakePoint(const float x, const float y, const float z)
{
	struct RLtuple p = { { x }, { y }, { z }, { 1.f } };
	return p;
}

struct RLtuple rlMakeVector(const float x, const float y, const float z)
{
	struct RLtuple v = { { x }, { y }, { z }, { 0.f } };
	return v;
}

struct RLtuple rlTadd(const struct RLtuple a, const struct RLtuple b)
{
	struct RLtuple sum = {
		{ a.x + b.x }, { a.y + b.y }, { a.z + b.z }, { a.w + b.w }
	};
	return sum;
}
