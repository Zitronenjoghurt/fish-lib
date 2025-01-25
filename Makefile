.PHONY: test-up test-run test-clean test

test-up:
	cd example && make up
	cd test && make up

test-run:
	cd example && make test
	cd test && make test

test-clean:
	cd example && make clean
	cd test && make clean

test: test-up test-run test-clean