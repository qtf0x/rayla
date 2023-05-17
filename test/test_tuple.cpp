#include <gtest/gtest.h>

extern "C" {
#include "tuple.h"
#include "utils.h"
}

// Scenario: A tuple with w=1.0 is a point
TEST(tuple, wOnePoint)
{
	// Given a <- tuple(4.3, -4.2, 3.1, 1.0)
	auto a{ rlMakeTuple(4.3f, -4.2f, 3.1f, 1.f) };

	// Then a.x = 4.3
	ASSERT_TRUE(rlFeq(a.x, 4.3f));
	// And a.y = -4.2
	ASSERT_TRUE(rlFeq(a.y, -4.2f));
	// And a.z = 3.1
	ASSERT_TRUE(rlFeq(a.z, 3.1f));
	// And a.w = 1.0
	ASSERT_TRUE(rlFeq(a.w, 1.f));

	// And a is a point
	ASSERT_TRUE(rlIsPoint(a));
	// And a is not a vector
	ASSERT_FALSE(rlIsVector(a));
}

// Scenario: A tuple with w=0 is a vector
TEST(tuple, wZeroVector)
{
	// Given a <- tuple(4.3, -4.2, 3.1, 0.0)
	auto a{ rlMakeTuple(4.3f, -4.2f, 3.1f, 0.f) };

	// Then a.x = 4.3
	ASSERT_TRUE(rlFeq(a.x, 4.3f));
	// And a.y = -4.2
	ASSERT_TRUE(rlFeq(a.y, -4.2f));
	// And a.z = 3.1
	ASSERT_TRUE(rlFeq(a.z, 3.1f));
	// And a.w = 1.0
	ASSERT_TRUE(rlFeq(a.w, 0.f));

	// And a is not a point
	ASSERT_FALSE(rlIsPoint(a));
	// And a is a vector
	ASSERT_TRUE(rlIsVector(a));
}

// Scenario: point() creates tupes with w=1
TEST(tuple, createPoint)
{
	// Given p <- point(4, -4, 3)
	auto p{ rlMakePoint(4.f, -4.f, 3.f) };

	// Then p = tuple(4, -4, 3, 1)
	ASSERT_TRUE(rlTeq(p, { 4.f, -4.f, 3.f, 1.f }));
}

// Scenario: vector() creates tuples with w=0
TEST(tuple, createVector)
{
	// Given v <- vector(4, -4, 3)
	auto v{ rlMakeVector(4.f, -4.f, 3.f) };

	// Then tuple(4, -4, 3, 0)
	ASSERT_TRUE(rlTeq(v, { 4.f, -4.f, 3.f, 0.f }));
}

// Scenario: Adding two tuples
TEST(tuple, addTuples)
{
	// Given a1 <- tuple(3, -2, 5, 1)
	auto a1{ rlMakeTuple(3.f, -2.f, 5.f, 1.f) };
	// And a2 <- tuple(-2, 3, 1, 0)
	auto a2{ rlMakeTuple(-2.f, 3.f, 1.f, 0.f) };

	// Then a1 + a2 = tuple(1, 1, 5, 1)
	ASSERT_TRUE(rlTeq(rlTadd(a1, a2), { 1.f, 1.f, 6.f, 1.f }));
}
