ifeq ($(OS),Windows_NT)
	CHILD := windows
else
	CHILD := unix
endif

%:
	@$(MAKE) -f build_utils/$(CHILD)/Makefile $@
