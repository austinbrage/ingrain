.PHONY: dev dev\:watch dev\:fresh test complete complete\:watch complete-long complete-long\:watch complete-session complete-session\:watch complete-module complete-module\:watch exercise exercise\:watch

dev:
	@cargo run -q

test:
	@cargo test -- --nocapture

dev\:watch:
	@exec watchexec -r -c -e rs -- cargo run -q

dev\:fresh:
	@rm -rf /tmp/ingrain-new
	@INGRAIN_DATA_DIR=/tmp/ingrain-new cargo run -q

complete:
	@cargo run -q -- complete

complete\:watch:
	@exec watchexec -r -c -e rs -- cargo run -q -- complete

complete-long:
	@cargo run -q -- complete-long

complete-long\:watch:
	@exec watchexec -r -c -e rs -- cargo run -q -- complete-long

complete-session:
	@cargo run -q -- session-done

complete-session\:watch:
	@exec watchexec -r -c -e rs -- cargo run -q -- session-done

complete-module:
	@cargo run -q -- module-done

complete-module\:watch:
	@exec watchexec -r -c -e rs -- cargo run -q -- module-done

exercise:
	@cargo run -q -- exercise $(path)

exercise\:watch:
	@exec watchexec -r -c -e rs -- cargo run -q -- exercise $(path)
