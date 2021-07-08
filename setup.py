from setuptools import setup
from setuptools_rust import RustExtension

setup(
    name="factor",
    version="0.1.0",
    classifiers=[
        "License :: OSI Approved :: MIT License",
        "Programming Language :: Python",
        "Programming Language :: Rust",
        "Operating System :: POSIX",
    ],
    packages=["factor"],
    rust_extensions=[RustExtension("factor.factor", "Cargo.toml", debug=False)],
    include_package_data=True,
    zip_safe=False,
)
