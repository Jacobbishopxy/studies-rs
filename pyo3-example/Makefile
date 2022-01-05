

deps:
	pip install maturin && pip install pydantic

build:
	maturin develop

check:
	pip show pyo3-example

test_sync:
	python pyo3_sync.py

test_async:
	python pyo3_async.py

