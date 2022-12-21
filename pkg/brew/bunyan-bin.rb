class BunyanView < Formula
  desc "A full-featured Rust port of the Node Bunyan structured log file viewer"
  homepage "https://github.com/dekobon/bunyan-view"
  version "0.3.1"

  if OS.mac? and Hardware::CPU.intel?
      url "https://github.com/dekobon/bunyan-view/releases/download/v#{version}/bunyan-v#{version}_x86_64-apple-darwin.tar.gz"
      sha256 ""
  elsif OS.mac? and Hardware::CPU.arm?
      url "https://github.com/dekobon/bunyan-view/releases/download/v#{version}/bunyan-v#{version}_aarch64-apple-darwin.tar.gz"
      sha256 ""
  elsif OS.linux? and Hardware::CPU.intel?
      url "https://github.com/dekobon/bunyan-view/releases/download/v#{version}/bunyan-v#{version}_x86_64-unknown-linux-gnu.tar.gz"
      sha256 "9a99fc2b49dc0b0cf3264cb7637ec05a22e9e6b8e74db6679bad0e3675989ec2"
  elsif OS.linux? and Hardware::CPU.arm? and Hardware::CPU.is_64_bit?
      url "https://github.com/dekobon/bunyan-view/releases/download/v#{version}/bunyan-v#{version}_aarch64-unknown-linux-gnu.tar.gz"
      sha256 "efa8dae2af9b585a3800400cea7834562647eaba2fe579849bffd072edfc71f2"
  else
      odie "Unsupported architecture"
  end


  def install
    bin.install "bunyan"
    man1.install "bunyan.1.gz"
  end
end
