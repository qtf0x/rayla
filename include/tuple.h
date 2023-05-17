/**
 * @file tuple.h
 * @author Vincent Marias ~ @qtf0x
 * @date 05/16/2023
 * 
 * @copyright MIT License Copyright (c) 2023 Vincent Marias
 * 
 * @brief Definition and functionality for a 4D tuple data structure.
 */

#ifndef RAYLA_TUPLE_H
#define RAYLA_TUPLE_H

#include <stdbool.h> // for bool

/**
 * @brief A 4D collection of floating-point numbers.
 */
struct RLtuple {
	// values for all four dimensions
	float x, y, z, w;
};

/**
 * @brief Determines whether a given tuple is a point.
 * 
 * @param tup the tuple to test
 * @return true the tuple is a point, with a W component of 1
 * @return false the tuple is not a point
 */
bool rlIsPoint(const struct RLtuple tup);

/**
 * @brief Determines whether a given tuple is a vector.
 * 
 * @param tup the tuple to test
 * @return true the tuple is a vector, with a W component of 0
 * @return false the tuple is not a point
 */
bool rlIsVector(const struct RLtuple tup);

/**
 * @brief Compares two tuples for element-wise equality.
 * 
 * @param a first tuple to compare
 * @param b second tuple to compare
 * @return true tuple elements "close-enough" to equal
 * @return false tuple elements not equal
 */
bool rlTeq(const struct RLtuple a, const struct RLtuple b);

/**
 * @brief Creates a point tuple from given xyz elements (W component 1).
 * 
 * @param x first element of point
 * @param y second element of point
 * @param z third element of point
 * @return struct RLtuple point constructed from from given xyz elements and 
 * with a W component of 1
 */
struct RLtuple rlPoint(const float x, const float y, const float z);

/**
 * @brief Creates a vector tuple from given xyz elements (W component 0).
 * 
 * @param x first element of vector
 * @param y second element of vector
 * @param z third element of vector
 * @return struct RLtuple vector constructed from from given xyz elements and 
 * with a W component of 0
 */
struct RLtuple rlVector(const float x, const float y, const float z);

/**
 * @brief Performs the element-wise sum of two tuples.
 * 
 * @param a first tuple to add
 * @param b second tuple to add
 * @return struct RLtuple element-wise sum of a and b
 */
struct RLtuple rlTadd(const struct RLtuple a, const struct RLtuple b);

#endif // RAYLA_TUPLE_H
