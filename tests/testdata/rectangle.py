from typing import Callable, Generic, TypeVar

T = TypeVar("T")


class Rectangle(Generic[T]):
    def __init__(self, width: T, height: T):
        self.width = width
        self.height = height

    # Method to calculate the area, requiring T to implement the mul operation
    def area(self, mul: Callable[[T, T], T]) -> T:
        return mul(self.width, self.height)

    # Method to calculate the perimeter, requiring T to implement the add operation
    def perimeter(self, add: Callable[[T, T], T]) -> T:
        return add(add(self.width, self.height), add(self.width, self.height))


def new_rectangle(width: T, height: T) -> Rectangle[T]:
    return Rectangle(width, height)


def main():
    int_mul = lambda a, b: a * b
    int_add = lambda a, b: a + b

    float_mul = lambda a, b: a * b
    float_add = lambda a, b: a + b

    rect1 = new_rectangle(3, 4)  # Rectangle with integers
    rect2 = new_rectangle(3.5, 4.5)  # Rectangle with floats

    print(f"Rectangle 1: {rect1.__dict__}")
    print(f"Area: {rect1.area(int_mul)}")
    print(f"Perimeter: {rect1.perimeter(int_add)}")

    print(f"Rectangle 2: {rect2.__dict__}")
    print(f"Area: {rect2.area(float_mul):.2f}")
    print(f"Perimeter: {rect2.perimeter(float_add):.2f}")


if __name__ == "__main__":
    main()
