# Rustlings: Learning Rust
> Containerized [rustlings](https://rustlings.cool/) code for learning Rust

If you'd like to use this container, used the `clean` branch for an empty repo

## Quickstart
1. Clone the repo
```bash
git clone -b clean git@github.com:dlg1206/rustlings.git
```

2. Build the image
```bash
docker build -t rustlings .
```

3. Run the container
```bash
docker run --rm -it -v "${PWD}/mount:/root/mount" --name rustlings rustlings
```
This will create a mounted directory to save your progress and have a container with rust while being able to use the
IDE of your choosing on the host machine

Note: to rerun a case, use l + enter + c