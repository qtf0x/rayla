/**
 * @file vec2.c
 * @author Vincent Marias ~ @qtf0x
 * @date 05/24/2023
 * 
 * @copyright MIT License Copyright (c) 2023 Vincent Marias
 * 
 * @brief Definitions for 2D vector type.
 */

#include <math.h> // for sqrtf

#include "vec2.h"
#include "rl_utils.h" // for feq

RLvec2 *rlVec2Init4f(RLvec2 *const v, const float x, const float y,
		     const float z)
{
	*v = (RLvec2){ { x }, { y } };

	return v;
}

RLvec2 *rlVec2Init1f(RLvec2 *const v, const float val)
{
	*v = (RLvec2){ { val }, { val } };

	return v;
}

float rlVec2Magnitude(const RLvec2 *const v)
{
	// pythagorean theorem
	return sqrtf(v->x * v->x + v->y * v->y);
}

bool rlVec2Equal(const RLvec2 *const v1, const RLvec2 *const v2)
{
	return feq(v1->x, v2->x) && feq(v1->y, v2->y);
}

RLvec2 *rlVec2Negate(RLvec2 *const neg, const RLvec2 *const v)
{
	*neg = (RLvec2){ { -v->x }, { -v->y } };

	return neg;
}

RLvec2 *rlVec2Add(RLvec2 *const sum, const RLvec2 *const augend,
		  const RLvec2 *const append)
{
	*sum = (RLvec2){ { augend->x + append->x }, { augend->y + append->y } };

	return sum;
}

RLvec2 *rlVec2Subtract(RLvec2 *const diff, const RLvec2 *const minuend,
		       const RLvec2 *const subtrahend)
{
	*diff = (RLvec2){ { minuend->x - subtrahend->x },
			  { minuend->y - subtrahend->y } };

	return diff;
}

RLvec2 *rlVec2Multiply(RLvec2 *const product, const float multiplier,
		       const RLvec2 *const multiplicand)
{
	*product = (RLvec2){ { multiplier * multiplicand->x },
			     { multiplier * multiplicand->y } };

	return product;
}

RLvec2 *rlVec2Normalize(RLvec2 *const norm, const RLvec2 *const v)
{
	float mag = rlVec2Magnitude(v);

	// avoid division by 0 (only matters if divisor exactly 0)
	if (mag == 0.f) {
		rlVec2Init1f(norm, 0.f);
	} else {
		*norm = (RLvec2){ { v->x / mag }, { v->y / mag } };
	}

	return norm;
}
