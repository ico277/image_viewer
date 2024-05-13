CXX = c++
CXXFILES = $(wildcard src/*.cpp)
CXXFLAGS = -O2
LDFLAGS = -lpluto
PREFIX = /usr/local
EXECUTABLE = image_viewer

.PHONY: build install uninstall clean run debug 

build:
	$(CXX) $(CXXFILES) -o $(EXECUTABLE).out $(LDFLAGS) $(CXXFLAGS)

install: ./$(EXECUTABLE)
	cp ./$(EXECUTABLE).out $(PREFIX)/bin/$(EXECUTABLE)

uninstall: $(PREFIX)/bin/$(EXECUTABLE)
	rm $(PREFIX)/bin/$(EXECUTABLE)

clean:
	rm ./*.out 2> /dev/null || true
	rm ./vgcore* 2> /dev/null || true

run: build
	./$(EXECUTABLE).out

debug: clean
	$(CXX) -DDEBUG $(CXXFILES) -o $(EXECUTABLE)_debug.out $(LDFLAGS) $(CXXFLAGS) -g
	./$(EXECUTABLE)_debug.out $(RUNARGS)

