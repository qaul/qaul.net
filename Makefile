.PHONY: bundle-snapfile
bundle-snapfile:
	@docker run --rm -v $(shell pwd):"/qaul.net" -w "/qaul.net" snapcore/snapcraft:edge /bin/bash -c 'sleep 1; cd utilities/installers/linux && bash snapbuild'
	@echo ""
