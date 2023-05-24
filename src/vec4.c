/**
 * @file vec4.c
 * @author Vincent Marias ~ @qtf0x
 * @date 05/24/2023
 * 
 * @copyright MIT License Copyright (c) 2023 Vincent Marias
 * 
 * @brief Definitions for 4D vector type.
 */

#include <math.h> // for sqrtf

#include "vec4.h"
#include "rl_utils.h" // for feq

RLvec4 *rlVec4Init4f(RLvec4 *const v, const float x, const float y,
		     const float z, const float w)
{
	*v = (RLvec4){ { x }, { y }, { z }, { w } };

	return v;
}

RLvec4 *rlVec4Init1f(RLvec4 *const v, const float val)
{
	*v = (RLvec4){ { val }, { val }, { val }, { val } };

	return v;
}

float rlVec4Magnitude(const RLvec4 *const v)
{
	// pythagorean theorem
	return sqrtf(v->x * v->x + v->y * v->y + v->z * v->z + v->w * v->w);
}

bool rlVec4Equal(const RLvec4 *const v1, const RLvec4 *const v2)
{
	return feq(v1->x, v2->x) && feq(v1->y, v2->y) && feq(v1->z, v2->z) &&
	       feq(v1->w, v2->w);
}

RLvec4 *rlVec4Negate(RLvec4 *const neg, const RLvec4 *const v)
{
	*neg = (RLvec4){ { -v->x }, { -v->y }, { -v->z }, { -v->w } };

	return neg;
}

RLvec4 *rlVec4Add(RLvec4 *const sum, const RLvec4 *const augend,
		  const RLvec4 *const append)
{
	*sum = (RLvec4){ { augend->x + append->x },
			 { augend->y + append->y },
			 { augend->z + append->z },
			 { augend->w + append->w } };

	return sum;
}

RLvec4 *rlVec4Subtract(RLvec4 *const diff, const RLvec4 *const minuend,
		       const RLvec4 *const subtrahend)
{
	*diff = (RLvec4){ { minuend->x - subtrahend->x },
			  { minuend->y - subtrahend->y },
			  { minuend->z - subtrahend->z },
			  { minuend->w - subtrahend->w } };

	return diff;
}

RLvec4 *rlVec4Multiply(RLvec4 *const product, const float multiplier,
		       const RLvec4 *const multiplicand)
{
	*product = (RLvec4){ { multiplier * multiplicand->x },
			     { multiplier * multiplicand->y },
			     { multiplier * multiplicand->z },
			     { multiplier * multiplicand->w } };

	return product;
}

RLvec4 *rlVec4Normalize(RLvec4 *const norm, const RLvec4 *const v)
{
	float mag = rlVec4Magnitude(v);

	// avoid division by 0 (only matters if divisor exactly 0)
	if (mag == 0.f) {
		rlVec4Init1f(norm, 0.f);
	} else {
		*norm = (RLvec4){ { v->x / mag },
				  { v->y / mag },
				  { v->z / mag },
				  { v->w / mag } };
	}

	return norm;
}

bool rlIsHomoVector(const RLvec4 *const tup)
{
	return feq(tup->w, 0.f);
}

bool rlIsHomoPoint(const RLvec4 *const tup)
{
	return feq(tup->w, 1.f);
}
