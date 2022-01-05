# We need maturin to be installed:
# `pip install maturin`
#
# To generate the new package source:
# `maturin develop`
#
# Other dependencies:
# `pip install pydantic`

import pyo3_starter
from pydantic import BaseModel


class Human(BaseModel):
    name: str
    age: int


if __name__ == "__main__":

    print(pyo3_starter.multiply(1, 2))
    print(pyo3_starter.get_fibonacci(20))

    rust_struct = pyo3_starter.RustStruct(data="Hello", vector=[255, 0, 0])
    rust_struct.extend_vector([1, 1, 1, 1])
    rust_struct.printer()

    jan = Human(name="Jan", age=20)
    print(jan.json())

    pyo3_starter.human_says_hi(jan.json())
