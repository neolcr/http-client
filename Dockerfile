FROM rust:latest

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    git \
    curl \
    build-essential \
    ripgrep \
    fd-find \
    unzip \
    nodejs \
    npm \
    zsh \
    iputils-ping \
    fonts-powerline \
    && rm -rf /var/lib/apt/lists/*
    
    
RUN curl -fL https://github.com/neovim/neovim/releases/download/v0.11.3/nvim-linux-x86_64.tar.gz -o nvim.tar.gz \
 && tar xzf nvim.tar.gz \
 && mv nvim-linux-x86_64 /opt/nvim \
 && ln -sf /opt/nvim/bin/nvim /usr/local/bin/nvim
 
 
RUN apt-get update && apt-get install -y mingw-w64

RUN rustup target add x86_64-pc-windows-gnu

RUN ln -s $(which fdfind) /usr/local/bin/fd

RUN rustup component add rustfmt clippy

RUN cargo install cargo-watch cargo-edit

RUN git clone https://github.com/LazyVim/starter /root/.config/nvim

RUN sh -c "$(curl -fsSL https://starship.rs/install.sh)" -- -y

RUN chsh -s /usr/bin/zsh

RUN echo 'eval "$(starship init zsh)"' >> /root/.zshrc

RUN mkdir -p /root/.config/nvim/lua/config && \
cat >> /root/.config/nvim/lua/config/keymaps.lua <<'EOF'
vim.keymap.set("i", "jj", "<Esc>")

vim.keymap.set({ "n", "t" }, "tt", function()
  Snacks.terminal("bash", {
    win = {
      style = "terminal",
    },
  })
end, { desc = "Terminal flotante" })
EOF

WORKDIR /projects

EXPOSE 8080

CMD ["zsh"]



# podman build -t rust-dev .
# WINDOWS podman run -it -p 8080:8080 -v ${PWD}:/projects -v $HOME/.gitconfig:/root/.gitconfig -v $HOME/.ssh:/root/.ssh rust-dev 
# WSL sudo docker run -it -p 8080:8080 -v "$(pwd)":/projects -v "$HOME/.ssh:/root/.ssh" rust-dev
# podman run --network=host -it -v ${PWD}:/projects -v $HOME/.gitconfig:/root/.gitconfig -v $HOME/.ssh:/root/.ssh rust-dev
# sudo docker start -i rust-dev
# podman exec -it rust-dev zsh
# cargo build --release --target x86_64-pc-windows-gnu
