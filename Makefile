.PHONY: dev dev\:watch dev\:fresh test complete-exercise complete-exercise\:watch complete-exercise-long complete-exercise-long\:watch complete-session complete-session\:watch complete-module complete-module\:watch complete-module-failed complete-module-failed\:watch exercise exercise\:watch intro intro\:watch

dev:
	@cargo run -q

test:
	@cargo test -- --nocapture

dev\:watch:
	@exec watchexec -r -c -e rs -- cargo run -q

dev\:fresh:
	@rm -rf /tmp/ingrain-new
	@INGRAIN_DATA_DIR=/tmp/ingrain-new cargo run -q

complete-exercise:
	@cargo run -q -- complete

complete-exercise\:watch:
	@exec watchexec -r -c -e rs -- cargo run -q -- complete

complete-exercise-long:
	@cargo run -q -- complete-long

complete-exercise-long\:watch:
	@exec watchexec -r -c -e rs -- cargo run -q -- complete-long

complete-session:
	@cargo run -q -- session-done

complete-session\:watch:
	@exec watchexec -r -c -e rs -- cargo run -q -- session-done

complete-module:
	@cargo run -q -- module-done

complete-module\:watch:
	@exec watchexec -r -c -e rs -- cargo run -q -- module-done

complete-module-failed:
	@cargo run -q -- module-failed

complete-module-failed\:watch:
	@exec watchexec -r -c -e rs -- cargo run -q -- module-failed

exercise:
	@cargo run -q -- exercise $(path)

exercise\:watch:
	@exec watchexec -r -c -e rs -- cargo run -q -- exercise $(path)

intro:
	@cargo run -q -- intro

intro\:watch:
	@exec watchexec -r -c -e rs -- cargo run -q -- intro
