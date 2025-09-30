# God I wish this didn't need to be a container,
# but the dx tool has no way to do offline builds as far as I can tell,
# and I don't want to make my own buildRustPackage for dioxus

# TODO: seperate build and runtime containers

FROM rust
WORKDIR /app

COPY . .

# Install `dx`
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall dioxus-cli --root /.cargo -y --force
ENV PATH="/.cargo/bin:$PATH"

RUN dx bundle --platform web --features web

WORKDIR /usr/local/app
ENTRYPOINT [ "/usr/local/app/server" ]