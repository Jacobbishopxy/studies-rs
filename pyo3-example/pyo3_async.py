# pyo3-asyncio
#
# we need `import asyncio` to execute async functions provided by pyo3

import asyncio
import pyo3_starter


async def main():
    # coroutine
    result = await pyo3_starter.my_coroutine()
    print(f"my_coroutine returned with: {result!r}")


if __name__ == "__main__":

    asyncio.run(main())
