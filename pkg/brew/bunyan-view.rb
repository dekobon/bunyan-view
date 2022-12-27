class BunyanView < Formula
  desc "A full-featured Rust port of the Node Bunyan structured log file viewer"
  homepage "https://github.com/dekobon/bunyan-view"
  version "0.3.3"
  package_name = "bunyan-view"
  src_repo = "https://github.com/dekobon/bunyan-view"

  if OS.mac? and Hardware::CPU.intel?
      url "#{src_repo}/releases/download/v#{version}/#{package_name}_v#{version}_x86_64-apple-darwin.tar.gz"
      sha256 "a3287d12b293e1f62522e0a05e4bbb6bdac7fdb9160f81c0ecd80e82455c4670"
  elsif OS.mac? and Hardware::CPU.arm?
      url "#{src_repo}/releases/download/v#{version}/#{package_name}_#{version}_aarch64-apple-darwin.tar.gz"
      sha256 "2364ada5a4377a5a8833d30c3d16d73bce9f26cca3a4569d8e575e308f48dab2"
  elsif OS.linux? and Hardware::CPU.intel?
      url "#{src_repo}/releases/download/v#{version}/#{package_name}_#{version}_x86_64-unknown-linux-gnu.tar.gz"
      sha256 "177463e0a7aefb1398a80415f086676555800ce96f100490d3afbf168a388b62"
  elsif OS.linux? and Hardware::CPU.arm? and Hardware::CPU.is_64_bit?
      url "#{src_repo}/releases/download/v#{version}/#{package_name}_#{version}_aarch64-unknown-linux-gnu.tar.gz"
      sha256 "e3ad7ce064f6e06e21779801b1d2aa28103822b34a131003396a7f127f7baf8e"
  else
      odie "Unsupported architecture"
  end


  def install
    bin.install "bunyan"
    man1.install "bunyan.1.gz"
  end
end
