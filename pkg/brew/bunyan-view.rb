class BunyanView < Formula
  desc "A full-featured Rust port of the Node Bunyan structured log file viewer"
  homepage "https://github.com/dekobon/bunyan-view"
  version "0.3.1"

  if OS.mac? and Hardware::CPU.intel?
      url "https://github.com/dekobon/bunyan-view/releases/download/v#{version}/bunyan-v#{version}_x86_64-apple-darwin.tar.gz"
      sha256 "50dbdaf9543477960df1038ae0cee50cba3fdd5a05da8cfa3e1c5ddc02aa54d7"
  elsif OS.mac? and Hardware::CPU.arm?
      url "https://github.com/dekobon/bunyan-view/releases/download/v#{version}/bunyan-v#{version}_aarch64-apple-darwin.tar.gz"
      sha256 "d22e50fc01010219076ae559c2ed12dccde14c835044fcfcf4ba49004d6da87e"
  elsif OS.linux? and Hardware::CPU.intel?
      url "https://github.com/dekobon/bunyan-view/releases/download/v#{version}/bunyan-v#{version}_x86_64-unknown-linux-gnu.tar.gz"
      sha256 "bc2bf834b3f42193e8930fb96654024909947529a03f81c8c07ef51c72988ab6"
  elsif OS.linux? and Hardware::CPU.arm? and Hardware::CPU.is_64_bit?
      url "https://github.com/dekobon/bunyan-view/releases/download/v#{version}/bunyan-v#{version}_aarch64-unknown-linux-gnu.tar.gz"
      sha256 "f654d069bb7bc4c7d71104fccf9f94f22c124736964b33063d90b57eb868b779"
  else
      odie "Unsupported architecture"
  end


  def install
    bin.install "bunyan"
    man1.install "bunyan.1.gz"
  end
end
