/**
 * @file tuple.c
 * @author Vincent Marias ~ @qtf0x
 * @date 05/16/2023
 * 
 * @copyright MIT License Copyright (c) 2023 Vincent Marias
 * 
 * @brief Implementations for tuple.h functions.
 */

#include <math.h> // for sqrtf

#include "tuple.h"
#include "rl_utils.h" // for rlFeq

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

struct RLtuple rlMakeTuple1(const float x)
{
	struct RLtuple tup = { { x }, { x }, { x }, { x } };

	return tup;
}

struct RLtuple rlMakePoint(const float x, const float y, const float z)
{
	struct RLtuple p = { { x }, { y }, { z }, { 1.f } };

	return p;
}

struct RLtuple rlMakePoint1(const float x)
{
	struct RLtuple p = { { x }, { x }, { x }, { 1.f } };

	return p;
}

struct RLtuple rlMakeVector(const float x, const float y, const float z)
{
	struct RLtuple v = { { x }, { y }, { z }, { 0.f } };

	return v;
}

struct RLtuple rlMakeVector1(const float x)
{
	struct RLtuple v = { { x }, { x }, { x }, { 0.f } };

	return v;
}

struct RLtuple rlTadd(const struct RLtuple a, const struct RLtuple b)
{
	struct RLtuple sum = {
		{ a.x + b.x }, { a.y + b.y }, { a.z + b.z }, { a.w + b.w }
	};

	return sum;
}

struct RLtuple rlTsub(const struct RLtuple a, const struct RLtuple b)
{
	struct RLtuple diff = {
		{ a.x - b.x }, { a.y - b.y }, { a.z - b.z }, { a.w - b.w }
	};

	return diff;
}

struct RLtuple rlTneg(const struct RLtuple tup)
{
	struct RLtuple neg = { { -tup.x }, { -tup.y }, { -tup.z }, { -tup.w } };

	return neg;
}

struct RLtuple rlFTmul(const float scl, const struct RLtuple tup)
{
	struct RLtuple prod = { { scl * tup.x },
				{ scl * tup.y },
				{ scl * tup.z },
				{ scl * tup.w } };

	return prod;
}

float rlTmag(const struct RLtuple v)
{
	return sqrtf(v.x * v.x + v.y * v.y + v.z * v.z); // pythagorean theorem
}

struct RLtuple rlTnrm(const struct RLtuple tup)
{
	float mag = rlTmag(tup);

	// avoid division by 0 (only matters if divisor exactly 0)
	if (mag == 0.f)
		return rlMakeTuple1(0.f);

	struct RLtuple norm = { { tup.x / mag },
				{ tup.y / mag },
				{ tup.z / mag },
				{ tup.w / mag } };

	return norm;
}
