
if [[ $OSTYPE == 'darwin'* ]]; then
    echo ">>>> Mac detected!"

    echo ">>>> Installing xcode dev tools...."
    xcode-select —install

    echo ">>>> Installing brew...."
    /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

    echo ">>>> Installing git and curl...."
    brew install git curl glib-openssl
fi

echo ">>>> Installing Rust...."
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

echo ">>>> Downloading...."
mkdir ~/.greatness-src
cd ~/.greatness-src
git clone https://github.com/IsaccBarker/greatness.git
cd greatness

echo ">>>>> Building...."
cargo install --release --path .

echo ">>>>> Done! <<<<<"
echo "run \`great\` to get started!"

