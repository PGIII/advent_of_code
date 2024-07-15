default:
	cmake --build --preset default

configure:
	cmake . --preset default
.PHONY: configure

clean:
	cmake --build --preset default --target clean
.PHONY: clean

test: default
	ctest --preset default	
.PHONY: test

full-clean: 
	rm -r build/*
.PHONY: full-clean
