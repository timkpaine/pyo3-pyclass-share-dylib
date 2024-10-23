# pyo3-pyclass-share-dylib
Experimenting with workarounds for https://github.com/PyO3/pyo3/issues/1444


## Structure
This project has two folders, `lib1-py` representing a dependency that has `pyo3`-compiled rust structures (in `lib1-rs`) exposed as python classes, and `lib2-py` which attempts to reference these from rust.

~~Currently, this does not work out of the box due to https://github.com/PyO3/pyo3/issues/1444~~
Currently, this tests the "workaround" in https://github.com/PyO3/pyo3/issues/1444



## How to build
- run `maturin build` in `lib1-py`
- copy target/debug/liblib1_py.{so,dylib} into `lib1/lib1.so`
- run `maturin build` in `lib2-py`
- copy target/debug/liblib2_py.{so,dylib} into `lib2/lib2.so`
- from `lib2` folder, run python

## Testing
`lib1` contains a python class called `MyThing`, which can hold a value:

```python
import lib1
mt = MyThing("ONE")
mt
# MyThing<ONE>
```

`lib2` contains a python class called `MyOtherThing` which should be able to return a `MyThing`

```python
import lib2
mot = lib2.MyOtherThing("ONE")
mot.thing
# MyThing<ONE>
```


## State
~~
There are currently two big hurdles:
- libstd is dynamically linked, but should really end up in `lib1`
- you can import `lib1` and use it, or you can import `lib2` and use it, but right now when you import one and then the other, you will have symbol issues due to the duplicate and mutually exclusive copies of `lib1_py.dylib`. You can test this by uncommenting the first line of `lib2.__init__`
~~
There is some discussion of a monorepo-only workaround, this is testing those.

## Target State
To be considered working, the following should work with no local references to libstd or tweaks to `DYLD_LIBRARY_PATH`:

```python
import lib1
import lib2
mt = lib1.MyThing("ONE")
mot = lib2.MyOtherThing("ONE")

assert mot.thing == mt == lib1.MyThing.ONE
```

## Where to look
~~`lib2/.cargo` has the code for tweaking the rpath to look into `../lib1` to find `lib1.dylib`. However, this doesnt matter as `Makefile` will copy the `lib1.dylib` from targets when running `make dev`, resulting in 2 disparate copies of `lib1.dylib`, one in `lib1` and one in `lib2`. The import of `lib1` before `lib2` forces the use of `lib1`'s copy, but there are symbol mismatch issues as explained above.~~


