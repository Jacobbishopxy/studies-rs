# We need maturin to be installed:
# `pip install maturin`
#
# To generate the new package source:
# `maturin develop`
#
# Other dependencies:
# `pip install pydantic`

import rust
from pydantic import BaseModel


class Human(BaseModel):
    name: str
    age: int


if __name__ == "__main__":

    # print(rust.multiply(1, 2))
    # print(rust.get_fibonacci(20))

    # rust_struct = rust.RustStruct(data="Hello", vector=[255, 0, 0])
    # rust_struct.extend_vector([1, 1, 1, 1])
    # rust_struct.printer()

    jan = Human(name="Jan", age=20)
    print(jan.json())

    rust.human_says_hi(jan.json())
