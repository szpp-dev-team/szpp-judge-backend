.PHONY = genall
genall:
	diesel migration revert --all
	diesel migration run
	diesel database setup
