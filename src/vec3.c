/**
 * @file vec3.c
 * @author Vincent Marias ~ @qtf0x
 * @date 05/24/2023
 * 
 * @copyright MIT License Copyright (c) 2023 Vincent Marias
 * 
 * @brief Definitions for 3D vector type.
 */

#include <math.h> // for sqrtf

#include "vec3.h"
#include "rl_utils.h" // for feq

RLvec3 *rlVec3Init4f(RLvec3 *const v, const float x, const float y,
		     const float z)
{
	*v = (RLvec3){ { x }, { y }, { z } };

	return v;
}

RLvec3 *rlVec3Init1f(RLvec3 *const v, const float val)
{
	*v = (RLvec3){ { val }, { val }, { val } };

	return v;
}

float rlVec3Magnitude(const RLvec3 *const v)
{
	// pythagorean theorem
	return sqrtf(v->x * v->x + v->y * v->y + v->z * v->z);
}

bool rlVec3Equal(const RLvec3 *const v1, const RLvec3 *const v2)
{
	return feq(v1->x, v2->x) && feq(v1->y, v2->y) && feq(v1->z, v2->z);
}

RLvec3 *rlVec3Negate(RLvec3 *const neg, const RLvec3 *const v)
{
	*neg = (RLvec3){ { -v->x }, { -v->y }, { -v->z } };

	return neg;
}

RLvec3 *rlVec3Add(RLvec3 *const sum, const RLvec3 *const augend,
		  const RLvec3 *const append)
{
	*sum = (RLvec3){ { augend->x + append->x },
			 { augend->y + append->y },
			 { augend->z + append->z } };

	return sum;
}

RLvec3 *rlVec3Subtract(RLvec3 *const diff, const RLvec3 *const minuend,
		       const RLvec3 *const subtrahend)
{
	*diff = (RLvec3){ { minuend->x - subtrahend->x },
			  { minuend->y - subtrahend->y },
			  { minuend->z - subtrahend->z } };

	return diff;
}

RLvec3 *rlVec3Multiply(RLvec3 *const product, const float multiplier,
		       const RLvec3 *const multiplicand)
{
	*product = (RLvec3){ { multiplier * multiplicand->x },
			     { multiplier * multiplicand->y },
			     { multiplier * multiplicand->z } };

	return product;
}

RLvec3 *rlVec3Normalize(RLvec3 *const norm, const RLvec3 *const v)
{
	float mag = rlVec3Magnitude(v);

	// avoid division by 0 (only matters if divisor exactly 0)
	if (mag == 0.f) {
		rlVec3Init1f(norm, 0.f);
	} else {
		*norm = (RLvec3){ { v->x / mag },
				  { v->y / mag },
				  { v->z / mag } };
	}

	return norm;
}
