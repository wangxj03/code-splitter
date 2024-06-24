package main

import (
	"fmt"
)

type Rectangle[T any] struct {
	Width  T
	Height T
}

func NewRectangle[T any](width, height T) *Rectangle[T] {
	return &Rectangle[T]{Width: width, Height: height}
}

// Method to calculate the area, requiring T to implement the Mul method
func (r *Rectangle[T]) Area(mul func(T, T) T) T {
	return mul(r.Width, r.Height)
}

// Method to calculate the perimeter, requiring T to implement the Add method
func (r *Rectangle[T]) Perimeter(add func(T, T) T) T {
	return add(add(r.Width, r.Height), add(r.Width, r.Height))
}

func main() {
	intMul := func(a, b int) int { return a * b }
	intAdd := func(a, b int) int { return a + b }

	floatMul := func(a, b float64) float64 { return a * b }
	floatAdd := func(a, b float64) float64 { return a + b }

	rect1 := NewRectangle(3, 4) // Rectangle with integers
	rect2 := NewRectangle(3.5, 4.5) // Rectangle with floats

	fmt.Printf("Rectangle 1: %+v\n", *rect1)
	fmt.Printf("Area: %d\n", rect1.Area(intMul))
	fmt.Printf("Perimeter: %d\n", rect1.Perimeter(intAdd))

	fmt.Printf("Rectangle 2: %+v\n", *rect2)
	fmt.Printf("Area: %.2f\n", rect2.Area(floatMul))
	fmt.Printf("Perimeter: %.2f\n", rect2.Perimeter(floatAdd))
}
