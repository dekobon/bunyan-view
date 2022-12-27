class BunyanView < Formula
  desc "A full-featured Rust port of the Node Bunyan structured log file viewer"
  homepage "https://github.com/dekobon/bunyan-view"
  version "0.3.2"
  package_name = "bunyan-view"
  src_repo = "https://github.com/dekobon/bunyan-view"

  if OS.mac? and Hardware::CPU.intel?
      url "#{src_repo}/releases/download/v#{version}/#{package_name}_v#{version}_x86_64-apple-darwin.tar.gz"
      sha256 ""
  elsif OS.mac? and Hardware::CPU.arm?
      url "#{src_repo}/releases/download/v#{version}/#{package_name}_#{version}_aarch64-apple-darwin.tar.gz"
      sha256 ""
  elsif OS.linux? and Hardware::CPU.intel?
      url "#{src_repo}/releases/download/v#{version}/#{package_name}_#{version}_x86_64-unknown-linux-gnu.tar.gz"
      sha256 "f763e8efb8205a50f0eac79bec5d4418915411ba813504b76c31c4d5f7f86b65"
  elsif OS.linux? and Hardware::CPU.arm? and Hardware::CPU.is_64_bit?
      url "#{src_repo}/releases/download/v#{version}/#{package_name}_#{version}_aarch64-unknown-linux-gnu.tar.gz"
      sha256 "b44bec7ee25bd6f1385ef1737ae28c5c5c0acbc08fb9c85677725c7924221edd"
  else
      odie "Unsupported architecture"
  end


  def install
    bin.install "bunyan"
    man1.install "bunyan.1.gz"
  end
end
