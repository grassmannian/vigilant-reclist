# vigilant-reclist

Recommendation engine, works by taking an input set of interesting items, looking for other sets in universe that have sufficiently large intersections with input, and then yeilds all elements of those sets not contained in the input.

Database is currently a static JSON file, served separately, this should be expanded and improved in the future. 

# Usage

Build the software in the normal manner for Cargo projects
```
cargo build
```

Prior to running the engine, the database must be served over http. 

```
cd database
python -m http.server
```

Once database is being served, run the program
```
cargo build
```
