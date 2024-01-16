package-install:
	cargo install --path .

test-games:
	cargo test --test test_games

test-coverage:
	cargo tarpaulin --test test_games

# --------- games ---------
brain_even:
	cargo run --bin brain_even

brain_prime:
	cargo run --bin brain_prime

brain_calc:
	cargo run --bin brain_calc

brain_gcd:
	cargo run --bin brain_gcd

brain_progression:
	cargo run --bin brain_progression
