try:
    cargo build
    target/debug/hug -- /bin/cat sample.log
